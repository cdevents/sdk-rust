mod generated;

pub use generated::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDEvent {
    pub context: Context,
    pub subject: Subject,
    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<serde_json::Value>,
    #[serde(
        rename = "customDataContentType",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_data_content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    version: String,
    id: String,
    #[serde(with = "http_serde::uri")]
    source: http::Uri,
    #[serde(rename = "type")]
    tpe: String,
    #[serde(with = "time::serde::rfc3339")]
    timestamp: time::OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use rstest::rstest;
    use std::fs;
    use std::path::PathBuf;

    #[rstest]
    fn for_each_example(#[files("../cdevents-spec/examples/*.json")] path: PathBuf) {
        let example_json: serde_json::Value = serde_json::from_str(
            fs::read_to_string(&path)
                .expect("to read file as string")
                .as_str(),
        )
        .expect("to parse as json");
        let cdevent: CDEvent =
            serde_json::from_value(example_json.clone()).expect("to parse as cdevent");
        let cdevent_json = serde_json::to_value(cdevent).expect("to convert into json");
        assert_json_eq!(example_json, cdevent_json);
    }
}
