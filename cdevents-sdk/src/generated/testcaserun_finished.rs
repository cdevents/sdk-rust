// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "environment",)]
    pub environment: Environment,
    #[serde(rename = "outcome",)]
    pub outcome: String,
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none",)]
    pub reason: Option<String>,
    #[serde(rename = "severity", default, skip_serializing_if = "Option::is_none",)]
    pub severity: Option<String>,
    #[serde(rename = "testCase", default, skip_serializing_if = "Option::is_none",)]
    pub test_case: Option<TestCase>,
    #[serde(rename = "testSuiteRun", default, skip_serializing_if = "Option::is_none",)]
    pub test_suite_run: Option<TestSuiteRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestSuiteRun {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestCase {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none",)]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none",)]
    pub r#type: Option<String>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none",)]
    pub uri: Option<crate::Uri>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none",)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<crate::UriReference>,
}

