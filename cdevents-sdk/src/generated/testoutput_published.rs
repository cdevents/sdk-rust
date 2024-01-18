// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "format",)]
    pub format: String,
    #[serde(rename = "outputType",)]
    pub output_type: String,
    #[serde(rename = "testCaseRun", default, skip_serializing_if = "Option::is_none",)]
    pub test_case_run: Option<TestCaseRun>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none", with = "crate::serde::uri_optional",)]
    pub uri: Option<fluent_uri::Uri<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCaseRun {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<String>,
}

