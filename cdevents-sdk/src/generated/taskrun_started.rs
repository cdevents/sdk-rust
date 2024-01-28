// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "pipelineRun", default, skip_serializing_if = "Option::is_none",)]
    pub pipeline_run: Option<ContentPipelineRun>,
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none",)]
    pub task_name: Option<String>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none",)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ContentPipelineRun {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<crate::UriReference>,
}
