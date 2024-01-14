// code generated by cdevents/sdk-rust/generator (subject.hbs)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none",)]
    pub pipeline_name: Option<String>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none",)]
    pub url: Option<String>,
}

