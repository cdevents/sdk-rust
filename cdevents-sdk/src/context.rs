use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub version: String,
    pub id: String,
    #[serde(with = "crate::serde::uri_reference")]
    pub source: fluent_uri::Uri<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(with = "crate::serde::datetime")]
    pub timestamp: time::OffsetDateTime,
}
