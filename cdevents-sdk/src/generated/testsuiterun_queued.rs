// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "environment",)]
    pub environment: Environment,
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none",)]
    pub test_suite: Option<TestSuite>,
    #[serde(rename = "trigger", default, skip_serializing_if = "Option::is_none",)]
    pub trigger: Option<Trigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Trigger {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none",)]
    pub ty: Option<String>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none",)]
    pub uri: Option<crate::Uri>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TestSuite {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none",)]
    pub name: Option<String>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none",)]
    pub url: Option<crate::Uri>,
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

