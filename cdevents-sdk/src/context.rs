use serde::{Deserialize, Serialize};

use crate::UriReference;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Context {
    pub version: String,
    pub id: String,
    pub source: UriReference,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(with = "crate::serde::datetime")]
    pub timestamp: time::OffsetDateTime,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            version: "0.3.0".into(),
            id: "00000000-0000-0000-0000-000000000000".into(),
            source: UriReference::default(),
            r#type: "dev.cdevents.undef.undef.0.0.0".into(),
            timestamp: time::OffsetDateTime::now_utc(),
        }
    }
}
