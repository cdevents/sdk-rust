use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Content, UriReference};

/// see <https://github.com/cdevents/spec/blob/main/spec.md#cdevent-subject>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    #[serde(rename = "content")]
    content: Content,
    #[serde(rename = "id")]
    id: String,
    #[serde(
        rename = "source",
        default,
        skip_serializing_if = "Option::is_none",
    )]
    source: Option<UriReference>,
    #[serde(rename = "type")]
    ty: String,
}

impl Subject {
    /// see <https://github.com/cdevents/spec/blob/main/spec.md#id-subject>
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn with_id<T>(mut self, id: T) -> Self
        where T: Into<String> {
        self.id = id.into();
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#source-subject>
    pub fn source(&self) -> &Option<UriReference> {
        &self.source
    }

    pub fn with_source(mut self, source: UriReference) -> Self {
        self.source = Some(source);
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#type-context>
    /// derived from content
    pub fn ty(&self) -> &str {
        &self.ty
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#content>
    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn from_json(ty: &str, json: serde_json::Value) -> Result<Self, serde_json::Error> {
        Ok(Subject {
            id: json["id"]
                .as_str()
                .ok_or_else(|| serde::de::Error::missing_field("id"))?
                .to_string(),
            ty: json["type"]
                .as_str()
                .ok_or_else(|| serde::de::Error::missing_field("type"))?
                .to_string(),
            source: match json["source"].as_str() {
                None => None,
                Some(s) => Some(
                    UriReference::from_str(s).map_err(serde::de::Error::custom)?,
                ),
            },
            content: Content::from_json(ty, json["content"].clone())?,
        })
    }
}

impl<T> From<T> for Subject where T: Into<Content>{
    fn from(content: T) -> Self {
        let content = content.into();
        let ty = content.ty().to_owned();
        Self {
            content,
            id: String::new(),
            source: None,
            ty,
        }
    }
}
