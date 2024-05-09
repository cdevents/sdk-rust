use serde::{Deserialize, Serialize};

use crate::{Id, Uri, UriReference};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Context {
    pub(crate) version: String,
    pub(crate) id: Id,
    pub(crate) source: UriReference,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    #[serde(with = "crate::serde::datetime")]
    pub(crate) timestamp: time::OffsetDateTime,
    #[serde(rename = "schemaUri", skip_serializing_if = "Option::is_none")]
    pub(crate) schema_uri: Option<Uri>,
    #[serde(rename = "chain_id", skip_serializing_if = "Option::is_none")]
    pub(crate) chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) links: Option<serde_json::Value>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            version: "0.3.0".into(),
            id: Id::default(),
            source: "/undef".try_into().expect("/undef is a valid uri-reference"),
            ty: "dev.cdevents.undef.undef.0.0.0".into(),
            timestamp: time::OffsetDateTime::now_utc(),
            schema_uri: None,
            chain_id: None,
            links: None,
        }
    }
}
