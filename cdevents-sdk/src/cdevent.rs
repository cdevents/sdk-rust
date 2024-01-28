use crate::{Context, Subject, UriReference};
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CDEvent {
    context: Context,
    subject: Subject,
    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    custom_data: Option<serde_json::Value>,
    #[serde(
        rename = "customDataContentType",
        skip_serializing_if = "Option::is_none"
    )]
    custom_data_content_type: Option<String>,
}

impl From<Subject> for CDEvent {
    fn from(subject: Subject) -> Self {
        let context = Context {
            ty: subject.ty().into(),
            ..Default::default()
        };
        Self {
            context,
            subject,
            custom_data: None,
            custom_data_content_type: None,
        }
    }
}

impl CDEvent {
    /// see <https://github.com/cdevents/spec/blob/main/spec.md#version>
    pub fn version(&self) -> &str {
        self.context.version.as_str()
    }

    pub fn with_version<T>(mut self, v: T) -> Self where T: Into<String> {
        self.context.version = v.into();
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#id-context>
    pub fn id(&self) -> &str {
        self.context.id.as_str()
    }

    pub fn with_id<T>(mut self, v: T) -> Self where T: Into<String> {
        self.context.id = v.into();
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#source-context>
    pub fn source(&self) -> &UriReference {
        &self.context.source
    }

    pub fn with_source(mut self, v: UriReference) -> Self {
        self.context.source = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#timestamp>
    pub fn timestamp(&self) -> &time::OffsetDateTime {
        &self.context.timestamp
    }

    pub fn with_timestamp(mut self, v: time::OffsetDateTime) -> Self {
        self.context.timestamp = v;
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#cdevent-subject>
    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#type-context>
    /// derived from subject.content
    pub fn ty(&self) -> &str {
        //self.context.ty()
        self.subject.ty()
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#customdata>
    pub fn custom_data(&self) -> &Option<serde_json::Value> {
        &self.custom_data
    }

    pub fn with_custom_data(mut self, custom_data: serde_json::Value) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// see <https://github.com/cdevents/spec/blob/main/spec.md#customdatacontenttype>
    pub fn custom_data_content_type(&self) -> &Option<String> {
        &self.custom_data_content_type
    }

    pub fn with_custom_data_content_type(
        mut self,
        custom_data_content_type: String,
    ) -> Self {
        self.custom_data_content_type = Some(custom_data_content_type);
        self
    }
}

impl<'de> Deserialize<'de> for CDEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "camelCase")]
        enum Field {
            Context,
            Subject,
            CustomData,
            CustomDataContentType,
        }

        struct CDEventVisitor;

        impl<'de> Visitor<'de> for CDEventVisitor {
            type Value = CDEvent;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CDEvent")
            }

            fn visit_map<V>(self, mut map: V) -> Result<CDEvent, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut context: Option<Context> = None;
                let mut subject_json: Option<serde_json::value::Value> = None;
                let mut custom_data = None;
                let mut custom_data_content_type = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Context => {
                            if context.is_some() {
                                return Err(de::Error::duplicate_field("context"));
                            }
                            context = Some(map.next_value()?);
                        }
                        Field::Subject => {
                            if subject_json.is_some() {
                                return Err(de::Error::duplicate_field("subject"));
                            }
                            subject_json = Some(map.next_value()?);
                        }
                        Field::CustomData => {
                            if custom_data.is_some() {
                                return Err(de::Error::duplicate_field("customData"));
                            }
                            custom_data = Some(map.next_value()?);
                        }
                        Field::CustomDataContentType => {
                            if custom_data_content_type.is_some() {
                                return Err(de::Error::duplicate_field("customDataContentType"));
                            }
                            custom_data_content_type = Some(map.next_value()?);
                        }
                    }
                }
                let context = context.ok_or_else(|| de::Error::missing_field("context"))?;
                let subject_json =
                    subject_json.ok_or_else(|| de::Error::missing_field("subject"))?;
                let subject =
                    Subject::from_json(&context.ty, subject_json).map_err(de::Error::custom)?;

                Ok(CDEvent {
                    context,
                    subject,
                    custom_data,
                    custom_data_content_type,
                })
            }
        }

        const FIELDS: &[&str] = &["context", "subject"];
        deserializer.deserialize_struct("CDEvent", FIELDS, CDEventVisitor)
    }
}

#[cfg(feature = "testkit")]
impl<> proptest::arbitrary::Arbitrary for CDEvent {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        (
            any::<Subject>(),
            "\\PC*",
            any::<Option<UriReference>>(),
        ).prop_map(|(subject, id, source)| {
            let mut cdevent = CDEvent::from(subject).with_id(id);
            if let Some(source) = source {
                cdevent = cdevent.with_source(source);
            }
            cdevent
        }).boxed()
    }
}

#[cfg(test)]
mod tests {
    use crate::CDEvent;
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[cfg(feature = "testkit")]
        fn arbitraries_are_json_valid(s in any::<CDEvent>()) {
            let json_str = serde_json::to_string(&s).unwrap();
            let actual = serde_json::from_str::<CDEvent>(&json_str).unwrap();
            assert_eq!(s, actual);
        }
    }
}