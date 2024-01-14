use serde::{Deserialize, Serialize};

use crate::Content;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    #[serde(rename = "content")]
    pub content: Content,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "source",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde::uri_reference_optional"
    )]
    pub source: Option<fluent_uri::Uri<String>>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl Subject {
    pub fn from_json(ty: &str, json: serde_json::Value) -> Result<Self, serde_json::Error> {
        Ok(Subject {
            id: json["id"]
                .as_str()
                .ok_or_else(|| serde::de::Error::missing_field("id"))?
                .to_string(),
            r#type: json["type"]
                .as_str()
                .ok_or_else(|| serde::de::Error::missing_field("type"))?
                .to_string(),
            source: match json["source"].as_str() {
                None => None,
                Some(s) => Some(
                    fluent_uri::Uri::parse_from(s.to_owned())
                        .map_err(|e| serde::de::Error::custom(e.1))?,
                ),
            },
            content: Content::from_json(ty, json["content"].clone())?,
        })
    }
}
