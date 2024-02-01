use anyhow::{Context, Result};
use clap::Parser;
use cruet::Inflector;
use handlebars::{DirectorySourceOptions, Handlebars};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

/// generator of part of the rust code of cdevents from spec
#[derive(Parser, Debug)]
struct Settings {
    /// directory with handlebars templates
    #[arg(long, default_value = "templates")]
    templates_dir: PathBuf,

    /// directory with json schemas of events to generate
    #[arg(long, default_value = "../cdevents-spec/schemas")]
    jsonschema_dir: PathBuf,

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

    let mut variants = vec![];
    let mut jsonfiles =
        std::fs::read_dir(settings.jsonschema_dir)?.collect::<Result<Vec<_>, _>>()?;
    jsonfiles.sort_by_key(|v| v.file_name());
    for entry in jsonfiles {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "json" {
                let json: Value = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
                let (variant_info, code) = generate_variant(&hbs, json)
                    .with_context(|| format!("failed to generate variant on {:?}", &path))?;
                let file = settings
                    .dest
                    .join(cruet::to_snake_case(&variant_info.rust_module))
                    .with_extension("rs");
                //TODO use a formatter like https://crates.io/crates/prettyplease?
                fs::write(file, code)?;
                variants.push(variant_info);
            }
        }
    }

    let (module_name, code) =
        generate_module(&hbs, &variants).with_context(|| "failed to generate module")?;
    let file = settings.dest.join(module_name).with_extension("rs");
    //TODO use a formatter like https://crates.io/crates/prettyplease?
    fs::write(file, code)?;

    Ok(())
}

fn generate_variant(hbs: &Handlebars, jsonschema: Value) -> Result<(VariantInfo, String)> {
    // let id = jsonschema["$id"]
    //     .as_str()
    //     .ok_or(anyhow!("$id not found or not a string"))
    //     .and_then(|s| url::Url::parse(s).with_context(|| format!("failed to parse: {}", s)))?;
    // let module_name = id
    //     .path_segments()
    //     .and_then(|v| v.last())
    //     .map(cruet::to_snake_case)
    //     .ok_or(anyhow!("no path in $id"))?
    //     .replace("_event", "");

    // extract module's name from `context.type` (and not from `$id`)
    let context_type = jsonschema["properties"]["context"]["properties"]["type"]["default"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let fragments = context_type.split('.').collect::<Vec<_>>();
    let rust_module = format!("{}_{}", fragments[2], fragments[3]).to_snake_case();
    let predicate = fragments[3].to_owned();
    // due to inconstency in case/format the subject could be not be extracted from the context.type (ty), jsonshema $id, spec filename (shema, examples)
    let subject = jsonschema["properties"]["subject"]["properties"]["type"]["default"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    let data = build_data_for_variants(jsonschema);
    let code = hbs.render("variant", &data)?;
    let variant_info = VariantInfo {
        context_type,
        rust_module,
        subject,
        predicate,
    };
    Ok((variant_info, code))
}

fn generate_module(hbs: &Handlebars, variants: &[VariantInfo]) -> Result<(String, String)> {
    let data = json!({
        "variants": variants
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
    predicate: String,
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
                            (Some(x), Some(1)) if x.ends_with("Id") => "crate::Id",
                            (Some(x), Some(1)) if x.ends_with("Name") => "crate::Name",
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
