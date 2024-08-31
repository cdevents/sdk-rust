use assert_json_diff::assert_json_eq;
use cdevents_sdk::CDEvent;
use rstest::*;
use std::{collections::HashMap, fs};
use std::path::PathBuf;
use proptest::prelude::*;
use boon::{Compiler, SchemaIndex, Schemas, UrlLoader};
use glob::glob;
use std::sync::OnceLock;

struct EventsSchemas {
    schemas: Schemas,
    mapping: HashMap<String, SchemaIndex>,
}

impl EventsSchemas {
    fn load() -> Self {
        let mut schemas = Schemas::new();
        let mut compiler = Compiler::new();
        let mut mapping = HashMap::new();

        //HACK to resolve invalid `$ref: "/schema/links/embeddedlinksarray.json"`
        compiler.use_loader(Box::new(HackUrlLoader{}));

        for entry in glob("../cdevents-specs/*/schemas/*.json").expect("Failed to read glob pattern") {
            let schemapath = entry.unwrap();
            //TODO avoid to read the schema twice (as json, then as jsonschema)
            let jsonschema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&schemapath).unwrap()).unwrap();
            let ty = jsonschema["properties"]["context"]["properties"]["type"]["default"].as_str()
                .unwrap_or_default()
                .to_string();
            mapping.entry(ty).or_insert_with(|| {
                let sch_index = compiler.compile(&schemapath.to_string_lossy(), &mut schemas);
                if let Err(err) = sch_index {
                    panic!("{err:#}"); //like a assert(false,...)
                }
                sch_index.unwrap()
            });
        }
        Self {
            schemas, mapping
        }
    }

    fn check_against_schema(&self, json: &serde_json::Value, ty: &str) {
        let sch_index = self.mapping.get(ty).unwrap_or_else(|| panic!("to have schema for {ty}"));
        let result = self.schemas.validate(json, *sch_index);
        if let Err(err) = result {
            panic!("{err}");
        }
    }
}

struct HackUrlLoader;

impl UrlLoader for HackUrlLoader {
    fn load(&self, url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let re = regex::Regex::new(r"https://cdevents.dev/(?<version>\d+\.\d+)\.\d+/schema/(?<path>.*)")?;
        if let Some(caps) = re.captures(url) {
            let path = format!("../cdevents-specs/spec-v{}/schemas/{}.json", &caps["version"], &caps["path"]);
            let jsonschema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
            Ok(jsonschema)
        } else if url.starts_with("https://cdevents.dev/schema/links/") {
            // HACK to fix a bug in specs 0.4.0
            // [Link's ref path needs an update for all the event schemas · Issue #211 · cdevents/spec](https://github.com/cdevents/spec/issues/211)
            let path = url.replace("https://cdevents.dev/schema", "../cdevents-specs/spec-v0.4/schemas/");
            let jsonschema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
            Ok(jsonschema)
        } else if url.starts_with("file://") {
            let path = url.replace("file://", "");
            let jsonschema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
            Ok(jsonschema)
        } else {
            Err(format!("fail to load {url}").into())
        }
    }
}
static EVENTS_SCHEMA_CELL: OnceLock<EventsSchemas> = OnceLock::new();

fn events_schemas() -> &'static EventsSchemas { 
    EVENTS_SCHEMA_CELL.get_or_init(EventsSchemas::load)
}

#[rstest]
fn can_serde_example(#[files("../cdevents-specs/spec-*/examples/*.json")] #[files("../cdevents-specs/spec-*/conformance/*.json")] path: PathBuf) {
    let example_txt = fs::read_to_string(path).expect("to read file as string");
    // HACK uri are stored ad http::Uri, they are "normalized" when serialized, so prenormalization to avoid failure like
    // json atoms at path ".subject.content.repository.source" are not equal:
    //     lhs:
    //         "https://example.org"
    //     rhs:
    //         "https://example.org/"
    // But it's not the case with fluent_uri::UriRef
    //
    // example_txt = example_txt.replace("\"https://example.org\"", "\"https://example.org/\"");

    let example_json: serde_json::Value =
        serde_json::from_str(&example_txt).expect("to parse as json");
    dbg!(&example_json);
    let cdevent: CDEvent =
        serde_json::from_value(example_json.clone()).expect("to parse as cdevent");
    dbg!(&cdevent);
    let cdevent_json = serde_json::to_value(cdevent).expect("to convert into json");
    dbg!(&cdevent_json);
    assert_json_eq!(example_json, cdevent_json);
}

#[rstest]
fn validate_example_against_schema(#[files("../cdevents-specs/spec-*/examples/*.json")] #[files("../cdevents-specs/spec-*/conformance/*.json")] path: PathBuf) {
    let events_schemas = events_schemas();
    let example_txt = fs::read_to_string(path).expect("to read file as string");
    let example_json: serde_json::Value =
        serde_json::from_str(&example_txt).expect("to parse as json");
    let ty = example_json["context"]["type"].as_str().expect("valid context.type in json");
    events_schemas.check_against_schema(&example_json, ty);
}

proptest! {
    #[test]
    #[cfg(feature = "testkit")]
    fn arbitraries_check_jsonschema(s in any::<CDEvent>()) {
        let events_schemas = events_schemas();
        let json = serde_json::to_value(&s).unwrap();
        events_schemas.check_against_schema(&json, s.ty());
    }
}
