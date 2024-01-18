use assert_json_diff::assert_json_eq;
use cdevents_sdk::CDEvent;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;

#[rstest]
fn for_each_example(#[files("../cdevents-spec/examples/*.json")] path: PathBuf) {
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
