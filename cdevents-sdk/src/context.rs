use serde::{Deserialize, Serialize};

use crate::UriReference;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Context {
    pub(crate) version: String,
    pub(crate) id: String,
    pub(crate) source: UriReference,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    #[serde(with = "crate::serde::datetime")]
    pub(crate) timestamp: time::OffsetDateTime,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            version: "0.3.0".into(),
            id: "00000000-0000-0000-0000-000000000000".into(),
            source: UriReference::default(),
            ty: "dev.cdevents.undef.undef.0.0.0".into(),
            timestamp: time::OffsetDateTime::now_utc(),
        }
    }
}
