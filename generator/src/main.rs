use anyhow::{anyhow, Context, Result};
use clap::Parser;
use handlebars::{handlebars_helper, DirectorySourceOptions, Handlebars};
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
    hbs.register_helper("type_of", Box::new(type_of));
    hbs.register_helper("normalize_ident", Box::new(normalize_ident));
    handlebars_misc_helpers::register(&mut hbs);
    hbs.register_templates_directory(settings.templates_dir, DirectorySourceOptions::default())?;

    fs::create_dir_all(&settings.dest)?;

    let mut subjects = vec![];
    let mut jsonfiles =
        std::fs::read_dir(settings.jsonschema_dir)?.collect::<Result<Vec<_>, _>>()?;
    jsonfiles.sort_by_key(|v| v.file_name());
    for entry in jsonfiles {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "json" {
                let json = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
                let (type_name, code) = generate_subject(&hbs, json)
                    .with_context(|| format!("failed to generate subject on {:?}", &path))?;
                let file = settings
                    .dest
                    .join(cruet::to_snake_case(&type_name))
                    .with_extension("rs");
                fs::write(file, code)?;
                subjects.push(type_name);
            }
        }
    }

    let (type_name, code) =
        generate_module(&hbs, &subjects).with_context(|| "failed to generate module")?;
    let file = settings
        .dest
        .join(cruet::to_snake_case(&type_name))
        .with_extension("rs");
    fs::write(file, code)?;

    Ok(())
}

fn generate_subject(hbs: &Handlebars, jsonschema: Value) -> Result<(String, String)> {
    let id = jsonschema["$id"]
        .as_str()
        .ok_or(anyhow!("$id not found or not a string"))
        .and_then(|s| url::Url::parse(s).with_context(|| format!("failed to parse: {}", s)))?;
    let type_name = id
        .path_segments()
        .and_then(|v| v.last())
        .map(cruet::to_class_case)
        .ok_or(anyhow!("no path in $id"))?
        .replace("Event", "Subject");
    let mut data = jsonschema.clone();
    data.as_object_mut().and_then(|m| {
        m.insert(
            "type_name".to_string(),
            serde_json::to_value(&type_name).unwrap(),
        )
    });
    let code = hbs.render("subject", &data)?;
    Ok((type_name.to_string(), code))
}

fn generate_module(hbs: &Handlebars, subjects: &[String]) -> Result<(String, String)> {
    let data = json!({
        "subjects": subjects
    });
    let code = hbs.render("mod", &data)?;
    Ok(("mod".to_string(), code))
}

//TODO helper to convert into type
//TODO helper to check if optionnal
handlebars_helper!(type_of: |field_name: Value, def: Value, required: Value| {
    let mut t = match def["type"].as_str() {
        Some("string") => "String",
        Some("object") => "serde_json::Map<String, serde_json::Value>",
        x => todo!("impl type {:?}", x),
    }.to_string();
    if required.as_array().map(|a| a.contains(&field_name)).unwrap_or(false) {
        t = format!("Option<{}>", t);
    }
    t
});

handlebars_helper!(normalize_ident: |v: Value| {
    match v.as_str() {
        Some("type") => "tpe",
        Some(x) => x,
        None => unimplemented!(),
    }
});
