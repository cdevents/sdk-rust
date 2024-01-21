// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "errors", default, skip_serializing_if = "Option::is_none",)]
    pub errors: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none",)]
    pub outcome: Option<String>,
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none",)]
    pub pipeline_name: Option<String>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none",)]
    pub url: Option<String>,
}

