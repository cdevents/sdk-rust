// code generated by cdevents/sdk-rust/generator (subject.hbs)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subject {
    #[serde(rename = "content")]
    pub content: Content,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Content {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "viewUrl", skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}

