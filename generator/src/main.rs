use anyhow::{Context, Result};
use clap::Parser;
use cruet::{to_class_case, Inflector};
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
                let (rust_module, context_type, code) = generate_variant(&hbs, json)
                    .with_context(|| format!("failed to generate variant on {:?}", &path))?;
                let file = settings
                    .dest
                    .join(cruet::to_snake_case(&rust_module))
                    .with_extension("rs");
                fs::write(file, code)?;
                variants.push(VariantInfo {
                    context_type,
                    rust_module,
                });
            }
        }
    }

    let (module_name, code) =
        generate_module(&hbs, &variants).with_context(|| "failed to generate module")?;
    let file = settings.dest.join(&module_name).with_extension("rs");
    fs::write(file, code)?;

    Ok(())
}

fn generate_variant(hbs: &Handlebars, jsonschema: Value) -> Result<(String, String, String)> {
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
    let module_name = format!("{}_{}", fragments[2], fragments[3]).to_snake_case();

    let data = build_data_for_variants(jsonschema);
    let code = hbs.render("variant", &data)?;
    Ok((module_name.to_string(), context_type, code))
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
    collect_structs(
        &mut structs,
        "content",
        &jsonschema["properties"]["subject"]["properties"]["content"],
    );
    structs.reverse();

    json!({
        "structs": structs,
        "jsonschema": jsonschema,
    })
}

type RustTypeName = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariantInfo {
    context_type: String,
    rust_module: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StructDef {
    type_name: RustTypeName,
    json_definition: Value,
    fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FieldDef {
    rust_name: String,
    serde_name: String,
    type_name: RustTypeName,
    is_optional: bool,
}

fn collect_structs(
    structs: &mut Vec<StructDef>,
    field_name: &str,
    json_definition: &Value,
) -> RustTypeName {
    match json_definition["type"].as_str() {
        Some("string") => "String".to_string(),
        Some("object") => match json_definition["properties"].as_object() {
            None => "serde_json::Map<String, serde_json::Value>".to_string(),
            Some(fields_kv) => {
                let required = json_definition["required"].as_array();
                let fields = fields_kv
                    .into_iter()
                    .map(|field| {
                        let serde_name = field.0.to_string();
                        let rust_name = if serde_name == "type" {
                            "r#type".to_string()
                        } else {
                            serde_name.to_snake_case()
                        };
                        let mut type_name = collect_structs(structs, &serde_name, field.1);
                        let field_name = json!(&serde_name);
                        let is_optional =
                            required.map(|a| !a.contains(&field_name)).unwrap_or(true);
                        if is_optional {
                            type_name = format!("Option<{}>", type_name);
                        }
                        FieldDef {
                            rust_name,
                            serde_name,
                            type_name,
                            is_optional,
                        }
                    })
                    .collect::<Vec<_>>();
                let type_name = to_class_case(field_name);
                structs.push(StructDef {
                    type_name: type_name.clone(),
                    fields,
                    json_definition: json_definition.clone(),
                });
                type_name
            }
        },
        Some(x) => todo!("impl for type='{}'", x),
        None => unimplemented!("expected key 'type' in field '{}'", field_name),
    }
}
