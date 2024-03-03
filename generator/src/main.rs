use anyhow::{Context, Result};
use clap::Parser;
use cruet::Inflector;
use glob::glob;
use handlebars::{DirectorySourceOptions, Handlebars};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{cmp::Ordering, collections::HashMap, fs, path::PathBuf};

/// generator of part of the rust code of cdevents from spec
#[derive(Parser, Debug)]
struct Settings {
    /// directory with handlebars templates
    #[arg(long, default_value = "templates")]
    templates_dir: PathBuf,

    /// glob pattern to find files with json schemas of events to generate
    #[arg(long, default_value = "../cdevents-specs/*/schemas/*.json")]
    jsonschemas: String,

    /// destination directory where to generate code
    #[arg(long, default_value = "../cdevents-sdk/src/generated")]
    dest: PathBuf,
}

fn main() -> Result<()> {
    let settings = Settings::parse();

    let mut hbs = Handlebars::new();
    hbs.set_strict_mode(true);
    hbs.register_escape_fn(handlebars::no_escape);
    //hbs.unregister_escape_fn();
    handlebars_misc_helpers::register(&mut hbs);
    hbs.register_templates_directory(settings.templates_dir, DirectorySourceOptions::default())?;

    if settings.dest.exists() {
        fs::remove_dir_all(&settings.dest)?;
    }
    fs::create_dir_all(&settings.dest)?;

    let mut variants = IndexMap::new();
    let mut variants_per_specs = VariantPerSpecs::new();

    let mut jsonfiles = glob(&settings.jsonschemas)?.collect::<Result<Vec<_>, _>>()?;
    jsonfiles.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    for path in jsonfiles {
        let json: Value = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
        let (variant_info, code) = generate_variant(&hbs, json)
            .with_context(|| format!("failed to generate variant on {:?}", &path))?;
        if let Some(code) = code {
            variants_per_specs.register(&variant_info);

            if !variants.contains_key(&variant_info.context_type) {
                // generate code for variant
                let file = settings
                    .dest
                    .join(cruet::to_snake_case(&variant_info.rust_module))
                    .with_extension("rs");
                //TODO use a formatter like https://crates.io/crates/prettyplease?
                fs::write(file, code)?;
                variants.insert(variant_info.context_type.to_owned(), variant_info);
            }
        }
    }

    let (module_name, code) = generate_module(
        &hbs,
        &variants.into_values().collect::<Vec<_>>(),
        &variants_per_specs,
    )
    .with_context(|| "failed to generate module")?;
    let file = settings.dest.join(module_name).with_extension("rs");
    //TODO use a formatter like https://crates.io/crates/prettyplease?
    fs::write(file, code)?;

    Ok(())
}

fn to_rust_module(subject: &str, predicate: &str, version: Option<&Version>) -> String {
    //HACK: ignore module for version with modifier (draft, beta, ...)
    if let Some(version) = version {
        format!(
            "{}_{}_{}_{}_{}",
            subject, predicate, version.major, version.minor, version.patch
        )
        .to_snake_case()
    } else {
        format!("{}_{}", subject, predicate).to_snake_case()
    }
}

fn generate_variant(hbs: &Handlebars, jsonschema: Value) -> Result<(VariantInfo, Option<String>)> {
    //e.g. "$id": "https://cdevents.dev/0.3.0/schema/branch-created-event"
    let spec_version = jsonschema["$id"]
        .as_str()
        .ok_or(anyhow::anyhow!("$id not found or not a string"))
        .and_then(|s| url::Url::parse(s).with_context(|| format!("failed to parse: {}", s)))?
        .path_segments()
        .ok_or(anyhow::anyhow!("$id not an url with a path"))?
        .next()
        .unwrap_or("0.0.0")
        .to_owned();

    // extract module's name from `context.type` (and not from `$id`)
    let context_type = jsonschema["properties"]["context"]["properties"]["type"]["default"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let fragments = context_type.split('.').collect::<Vec<_>>();
    let subject = fragments[2].to_owned();
    let predicate = fragments[3].to_owned();
    let version_modifiers = fragments[6].split('-').collect::<Vec<_>>();

    let version = Version {
        major: fragments[4].parse::<u8>()?,
        minor: fragments[5].parse::<u8>()?,
        patch: version_modifiers[0].parse::<u8>()?,
        modifier: version_modifiers
            .get(1)
            .map(|s| s.to_string())
            .unwrap_or_default(),
    };
    let rust_module = to_rust_module(&subject, &predicate, Some(&version));
    // due to inconstency in case/format the subject could be not be extracted from the context.type (ty), jsonshema $id, spec filename (shema, examples)
    let subject_type = jsonschema["properties"]["subject"]["properties"]["type"]["default"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let data = build_data_for_variants(jsonschema);
    let code = if version.modifier.is_empty() {
        Some(hbs.render("variant", &data)?)
    } else {
        None
    };
    let variant_info = VariantInfo {
        context_type,
        rust_module,
        subject,
        subject_type,
        predicate,
        version,
        spec_version,
    };
    Ok((variant_info, code))
}

fn generate_module(
    hbs: &Handlebars,
    variants: &[VariantInfo],
    variants_per_specs: &VariantPerSpecs,
) -> Result<(String, String)> {
    let data = json!({
        "variants": variants,
        "variants_per_specs": variants_per_specs.generate_mapping(),
    });
    let code = hbs.render("mod", &data)?;
    Ok(("mod".to_string(), code))
}

fn build_data_for_variants(jsonschema: Value) -> Value {
    let mut structs = vec![];
    let mut enums = vec![];
    collect_structs(
        &mut structs,
        &mut enums,
        &["content"],
        &jsonschema["properties"]["subject"]["properties"]["content"],
    );
    structs.reverse();

    json!({
        "structs": structs,
        "enums": enums,
        "jsonschema": jsonschema,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct TypeInfo {
    type_declaration: String,
    serde_with: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariantInfo {
    context_type: String,
    rust_module: String,
    subject: String,
    subject_type: String,
    predicate: String,
    version: Version,
    spec_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    modifier: String,
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut r = self.major.cmp(&other.major);
        if r.is_eq() {
            r = self.minor.cmp(&other.minor);
        }
        if r.is_eq() {
            r = self.patch.cmp(&other.patch);
        }
        //TODO compare modifier with semantic ("snapshot", "draft", "alpha", "beta", "rc", "ga", "")
        r
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StructDef {
    type_info: TypeInfo,
    json_definition: Value,
    fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EnumDef {
    type_info: TypeInfo,
    json_definition: Value,
    /// (is_default, value as string)
    values: Vec<(bool, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FieldDef {
    rust_name: String,
    serde_name: String,
    type_info: TypeInfo,
    is_optional: bool,
}

fn collect_structs(
    structs: &mut Vec<StructDef>,
    enums: &mut Vec<EnumDef>,
    field_names: &[&str],
    json_definition: &Value,
) -> TypeInfo {
    match json_definition["type"].as_str() {
        Some("string") => match json_definition["format"].as_str() {
            Some("date-time") => TypeInfo {
                type_declaration: "time::OffsetDateTime".to_string(),
                serde_with: Some("crate::serde::datetime".to_string()),
            },
            //TODO manage purl
            Some("uri-reference") => TypeInfo {
                type_declaration: "crate::UriReference".to_string(),
                serde_with: None,
            },
            Some("uri") => TypeInfo {
                type_declaration: "crate::Uri".to_string(),
                serde_with: None,
            },
            _ => match json_definition["enum"].as_array() {
                None => {
                    let type_declaration =
                        match (field_names.last(), json_definition["minLength"].as_i64()) {
                            (Some(&"id"), _) => "crate::Id",
                            (Some(&"name"), Some(1)) => "crate::Name",
                            (Some(&"url"), Some(1)) => "crate::Uri", // workaround for wrongly typed in jsonschema
                            (Some(x), Some(1)) if x.ends_with("Id") => "crate::Id",
                            (Some(x), Some(1)) if x.ends_with("Name") => "crate::Name",
                            (Some(_), Some(1)) => "crate::NonEmptyString",
                            _ => "String",
                        }
                        .to_string();
                    TypeInfo {
                        type_declaration,
                        ..Default::default()
                    }
                }
                Some(values) => {
                    let default_value = json_definition["default"].as_str();
                    let values = values
                        .iter()
                        .map(|v| v.as_str().unwrap_or_default().to_string())
                        .map(|v| (default_value == Some(&v), v))
                        .collect::<Vec<_>>();
                    let type_info = TypeInfo {
                        type_declaration: to_type_name(field_names),
                        ..Default::default()
                    };
                    enums.push(EnumDef {
                        type_info: type_info.clone(),
                        json_definition: json_definition.clone(),
                        values,
                    });
                    type_info
                }
            },
        },
        Some("object") => match json_definition["properties"].as_object() {
            None => TypeInfo {
                type_declaration: "serde_json::Map<String, serde_json::Value>".to_string(),
                ..Default::default()
            },
            Some(fields_kv) => {
                let required = json_definition["required"].as_array();
                let fields = fields_kv
                    .into_iter()
                    .map(|field| {
                        let serde_name = field.0.to_string();
                        let rust_name = if serde_name == "type" {
                            "ty".to_string()
                        } else {
                            serde_name.to_snake_case()
                        };
                        let mut children_field_names = vec![];
                        children_field_names.extend_from_slice(field_names);
                        children_field_names.push(&serde_name);
                        let mut type_info =
                            collect_structs(structs, enums, &children_field_names, field.1);
                        let field_name = json!(&serde_name);
                        let is_optional =
                            required.map(|a| !a.contains(&field_name)).unwrap_or(true);
                        if is_optional {
                            type_info.type_declaration =
                                format!("Option<{}>", type_info.type_declaration);
                        }
                        FieldDef {
                            rust_name,
                            serde_name,
                            type_info,
                            is_optional,
                        }
                    })
                    .collect::<Vec<_>>();
                let type_info = TypeInfo {
                    type_declaration: to_type_name(field_names),
                    ..Default::default()
                };
                structs.push(StructDef {
                    type_info: type_info.clone(),
                    fields,
                    json_definition: json_definition.clone(),
                });
                type_info
            }
        },
        Some(x) => todo!("impl for type='{}'", x),
        None => unimplemented!("expected key 'type' in field '{}'", field_names.join(".")),
    }
}

fn to_type_name(fied_names: &[&str]) -> String {
    fied_names.join("_").to_class_case()
}

struct VariantPerSpecs {
    specs: HashMap<String, IndexMap<(String, String), Version>>,
}

impl VariantPerSpecs {
    fn new() -> Self {
        let mut specs = HashMap::new();
        specs.insert("latest".to_owned(), IndexMap::new());
        Self { specs }
    }

    fn register(&mut self, variant_info: &VariantInfo) {
        let key_version = (variant_info.subject.clone(), variant_info.predicate.clone());
        match self.specs.get_mut(&variant_info.spec_version) {
            Some(l) => {
                l.insert(key_version.clone(), variant_info.version.clone());
            }
            None => {
                let mut l = IndexMap::new();
                l.insert(key_version.clone(), variant_info.version.clone());
                self.specs.insert(variant_info.spec_version.clone(), l);
            }
        }

        if let Some(variants_latest) = self.specs.get_mut("latest") {
            // update variant_version
            match variants_latest.get(&key_version) {
                Some(v) if v < &variant_info.version => {
                    variants_latest.insert(key_version, variant_info.version.clone());
                }
                None => {
                    variants_latest.insert(key_version, variant_info.version.clone());
                }
                _ => (),
            }
        }
    }

    fn generate_mapping(&self) -> HashMap<String, Vec<(String, String)>> {
        self.specs.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            let mapping = v
                .iter()
                .map(|(k, v)| {
                    (
                        to_rust_module(&k.0, &k.1, Some(v)),
                        to_rust_module(&k.0, &k.1, None),
                    )
                })
                .collect::<Vec<_>>();
            let mod_name = if k == "latest" {
                k.to_owned()
            } else {
                format!("spec_{}", k).to_snake_case()
            };
            acc.insert(mod_name, mapping);
            acc
        })
    }
}
