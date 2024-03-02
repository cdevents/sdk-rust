use assert_json_diff::assert_json_eq;
use cdevents_sdk::CDEvent;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;
use proptest::prelude::*;
use boon::{Schemas, Compiler};

#[rstest]
fn for_each_example(#[files("../cdevents-specs/spec-v0.3/examples/*.json")] path: PathBuf) {
    let example_txt = fs::read_to_string(path).expect("to read file as string");
    //HACK uri are stored ad http::Uri, they are "normalized" when serialized, so prenormalization to avoid failure like
    // json atoms at path ".subject.content.repository.source" are not equal:
    //     lhs:
    //         "https://example.org"
    //     rhs:
    //         "https://example.org/"
    // But it's not the case with fluent_uri::Uri
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

fn check_against_schema(json: &serde_json::Value, ty: &str) {
    let (subject, predicate) = cdevents_sdk::extract_subject_predicate(ty).expect("valid type: {ty}");
    let schemapath = format!("../cdevents-specs/spec-v0.3/schemas/{subject}{predicate}.json").to_lowercase();
    //TODO optimize to not recompile a previously read schema
    let mut schemas = Schemas::new();
    let mut compiler = Compiler::new();
    let sch_index = compiler.compile(&schemapath, &mut schemas);
    if let Err(err) = sch_index {
        panic!("{err:#}"); //like a assert(false,...)
    }
    let sch_index = sch_index.unwrap();
    let result = schemas.validate(json, sch_index);
    if let Err(err) = result {
        panic!("{err}");
    }
}

#[rstest]
fn validate_example_against_schema(#[files("../cdevents-specs/spec-v0.3/examples/*.json")] path: PathBuf) {
    let example_txt = fs::read_to_string(path).expect("to read file as string");
    let example_json: serde_json::Value =
        serde_json::from_str(&example_txt).expect("to parse as json");
    let ty = example_json["context"]["type"].as_str().expect("valid context.type in json");
    check_against_schema(&example_json, ty);
}

proptest! {
    #[test]
    #[cfg(feature = "testkit")]
    fn arbitraries_check_jsonschema(s in any::<CDEvent>()) {
        let json = serde_json::to_value(&s).unwrap();
        check_against_schema(&json, s.ty());
    }
}
