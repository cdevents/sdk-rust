use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Content, Id, UriReference};

/// see <https://github.com/cdevents/spec/blob/main/spec.md#cdevent-subject>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    #[serde(rename = "content")]
    content: Content,
    #[serde(rename = "id")]
    id: Id,
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
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
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
                .try_into()
                .map_err(serde::de::Error::custom)?,
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
        let ty = crate::extract_subject_predicate(content.ty())
            .map(|(s, _)| s)
            .unwrap_or("unknown")
            .to_owned();
        Self {
            content,
            id: Id::default(),
            source: None,
            ty,
        }
    }
}

#[cfg(feature = "testkit")]
impl<> proptest::arbitrary::Arbitrary for Subject {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        (
            any::<Content>(),
            any::<Id>(),
            any::<Option<UriReference>>(),
        ).prop_map(|(content, id, source)| {
            let mut subject = Subject::from(content).with_id(id);
            if let Some(source) = source {
                subject = subject.with_source(source);
            }
            subject
        }).boxed()
    }
}


#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        #[cfg(feature = "testkit")]
        fn jsonify_arbitraries(s in any::<Subject>()) {
            // Not enough information into json of subject to deserialize into the same sut content
            // so only check that it could be serialized
            serde_json::to_string(&s).unwrap();
        }
    }
}