// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "environment",)]
    pub environment: ContentEnvironment,
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none",)]
    pub test_suite: Option<ContentTestSuite>,
    #[serde(rename = "trigger", default, skip_serializing_if = "Option::is_none",)]
    pub trigger: Option<ContentTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentTrigger {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none",)]
    pub ty: Option<ContentTriggerType>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none",)]
    pub uri: Option<crate::Uri>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentTestSuite {
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
pub struct ContentEnvironment {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<crate::UriReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ContentTriggerType {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "pipeline")]
    Pipeline,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "other")]
    Other,
}
