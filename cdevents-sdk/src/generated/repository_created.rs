// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "name",)]
    pub name: String,
    #[serde(rename = "owner", default, skip_serializing_if = "Option::is_none",)]
    pub owner: Option<String>,
    #[serde(rename = "url",)]
    pub url: String,
    #[serde(rename = "viewUrl", default, skip_serializing_if = "Option::is_none",)]
    pub view_url: Option<String>,
}

