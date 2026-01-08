#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

use crate::{Id, Uri, UriReference};

#[enum_dispatch::enum_dispatch]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged, deny_unknown_fields)]
pub(crate) enum ContextEnum {
    Context_0_5,
    Context_0_4,
    Context_0_3,
}

#[enum_dispatch::enum_dispatch(ContextEnum)]
pub trait Context {
    fn version(&self) -> &str;
    fn id(&self) -> &Id;
    fn with_id(self, v: Id) -> Self;
    /// see <https://github.com/cdevents/spec/blob/main/spec.md#source-context>
    fn source(&self) -> &UriReference;
    fn with_source(self, v: UriReference) -> Self;
        /// see <https://github.com/cdevents/spec/blob/main/spec.md#timestamp>
    fn timestamp(&self) -> &time::OffsetDateTime;
    fn with_timestamp(self, v: time::OffsetDateTime) -> Self;
    /// see <https://github.com/cdevents/spec/blob/main/spec.md#type-context>
    /// derived from subject.content
    fn ty(&self) -> &str;
}

impl ContextEnum {
    pub fn from_json(json: serde_json::value::Value) -> Result<Self, serde_json::Error> {
        let context = match json["specversion"].as_str().or_else(|| json["version"].as_str()) {
            Some("0.3.0") => Self::Context_0_3(serde_json::from_value(json)?),
            Some("0.4.0") | Some("0.4.1") => Self::Context_0_4(serde_json::from_value(json)?),
            _ => Self::Context_0_5(serde_json::from_value(json)?),
        };
        Ok(context)
    }
}

impl Default for ContextEnum {
    fn default() -> Self {
        Self::Context_0_5(Context_0_5 {
            specversion: "0.5.0".into(),
            id: Id::default(),
            source: "/undef".try_into().expect("/undef is a valid uri-reference"),
            ty: "dev.cdevents.undef.undef.0.0.0".into(),
            timestamp: time::OffsetDateTime::now_utc(),
            schema_uri: None,
            chain_id: None,
            links: None,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Context_0_5 {
    pub(crate) specversion: String,
    pub(crate) id: Id,
    pub(crate) source: UriReference,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    #[serde(with = "crate::serde::datetime")]
    pub(crate) timestamp: time::OffsetDateTime,
    #[serde(rename = "schemaUri", skip_serializing_if = "Option::is_none")]
    pub(crate) schema_uri: Option<Uri>,
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub(crate) chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) links: Option<serde_json::Value>,
}

impl Context_0_5 {
    pub(crate) fn new(ty: String) -> Self {
        Self{
            specversion: "0.5.0".into(),
            id: Id::default(),
            source: "/undef".try_into().expect("/undef is a valid uri-reference"),
            ty,
            timestamp: time::OffsetDateTime::now_utc(),
            schema_uri: None,
            chain_id: None,
            links: None,
        }
    }
}

impl Context for Context_0_5 {
    fn version(&self) -> &str {
        self.specversion.as_str()
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#id-context>
    fn id(&self) -> &Id {
        &self.id
    }

    fn with_id(mut self, v: Id) -> Self {
        self.id = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#source-context>
    fn source(&self) -> &UriReference {
        &self.source
    }

    fn with_source(mut self, v: UriReference) -> Self {
        self.source = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#timestamp>
    fn timestamp(&self) -> &time::OffsetDateTime {
        &self.timestamp
    }

    fn with_timestamp(mut self, v: time::OffsetDateTime) -> Self {
        self.timestamp = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#type-context>
    /// derived from subject.content
    fn ty(&self) -> &str {
        //self.subject.content().ty()
        self.ty.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Context_0_4 {
    pub(crate) version: String,
    pub(crate) id: Id,
    pub(crate) source: UriReference,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    #[serde(with = "crate::serde::datetime")]
    pub(crate) timestamp: time::OffsetDateTime,
    #[serde(rename = "schemaUri", skip_serializing_if = "Option::is_none")]
    pub(crate) schema_uri: Option<Uri>,
    #[serde(rename = "chainId", skip_serializing_if = "Option::is_none")]
    pub(crate) chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) links: Option<serde_json::Value>,
}

impl Context_0_4 {
    pub(crate) fn new(ty: String) -> Self {
        Self{
            version: "0.4.1".into(),
            id: Id::default(),
            source: "/undef".try_into().expect("/undef is a valid uri-reference"),
            ty,
            timestamp: time::OffsetDateTime::now_utc(),
            schema_uri: None,
            chain_id: None,
            links: None,
        }
    }
}

impl Context for Context_0_4 {
    fn version(&self) -> &str {
        self.version.as_str()
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#id-context>
    fn id(&self) -> &Id {
        &self.id
    }

    fn with_id(mut self, v: Id) -> Self {
        self.id = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#source-context>
    fn source(&self) -> &UriReference {
        &self.source
    }

    fn with_source(mut self, v: UriReference) -> Self {
        self.source = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#timestamp>
    fn timestamp(&self) -> &time::OffsetDateTime {
        &self.timestamp
    }

    fn with_timestamp(mut self, v: time::OffsetDateTime) -> Self {
        self.timestamp = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#type-context>
    /// derived from subject.content
    fn ty(&self) -> &str {
        //self.subject.content().ty()
        self.ty.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Context_0_3 {
    pub(crate) version: String,
    pub(crate) id: Id,
    pub(crate) source: UriReference,
    #[serde(rename = "type")]
    pub(crate) ty: String,
    #[serde(with = "crate::serde::datetime")]
    pub(crate) timestamp: time::OffsetDateTime,
}

impl Context_0_3 {
    pub(crate) fn new(ty: String) -> Self {
        Self{
            version: "0.3.0".into(),
            id: Id::default(),
            source: "/undef".try_into().expect("/undef is a valid uri-reference"),
            ty,
            timestamp: time::OffsetDateTime::now_utc(),
        }
    }
}

impl Context for Context_0_3 {
    fn version(&self) -> &str {
       self.version.as_str()
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#id-context>
    fn id(&self) -> &Id {
        &self.id
    }

    fn with_id(mut self, v: Id) -> Self {
        self.id = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#source-context>
    fn source(&self) -> &UriReference {
        &self.source
    }

    fn with_source(mut self, v: UriReference) -> Self {
        self.source = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#timestamp>
    fn timestamp(&self) -> &time::OffsetDateTime {
        &self.timestamp
    }

    fn with_timestamp(mut self, v: time::OffsetDateTime) -> Self {
        self.timestamp = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#type-context>
    /// derived from subject.content
    fn ty(&self) -> &str {
        //self.subject.content().ty()
        self.ty.as_str()
    }
}
