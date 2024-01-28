// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "pipelineName",)]
    pub pipeline_name: String,
    #[serde(rename = "url",)]
    pub url: String,
}
