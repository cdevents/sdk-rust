// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "environment",)]
    pub environment: ContentEnvironment,
    #[serde(rename = "outcome",)]
    pub outcome: ContentOutcome,
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none",)]
    pub reason: Option<String>,
    #[serde(rename = "severity", default, skip_serializing_if = "Option::is_none",)]
    pub severity: Option<ContentSeverity>,
    #[serde(rename = "testCase", default, skip_serializing_if = "Option::is_none",)]
    pub test_case: Option<ContentTestCase>,
    #[serde(rename = "testSuiteRun", default, skip_serializing_if = "Option::is_none",)]
    pub test_suite_run: Option<ContentTestSuiteRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentTestSuiteRun {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentTestCase {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none",)]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none",)]
    pub ty: Option<ContentTestCaseType>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none",)]
    pub uri: Option<crate::Uri>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none",)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentEnvironment {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<crate::UriReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ContentOutcome {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "fail")]
    Fail,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ContentSeverity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ContentTestCaseType {
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "functional")]
    Functional,
    #[serde(rename = "unit")]
    Unit,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "compliance")]
    Compliance,
    #[serde(rename = "integration")]
    Integration,
    #[serde(rename = "e2e")]
    E2E,
    #[serde(rename = "other")]
    Other,
}
