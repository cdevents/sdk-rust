mod generated;

pub use generated::*;
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CDEvent {
    pub context: Context,
    pub subject: Subject,
    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<serde_json::Value>,
    #[serde(
        rename = "customDataContentType",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_data_content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    version: String,
    id: String,
    #[serde(with = "http_serde::uri")]
    source: http::Uri,
    #[serde(rename = "type")]
    r#type: String,
    #[serde(with = "time::serde::rfc3339")]
    timestamp: time::OffsetDateTime,
}

impl<'de> Deserialize<'de> for CDEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Context,
            Subject,
            CustomData,
            CustomDataContentType,
        }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { { Context, Subject, CustomData, CustomDataContentType } }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`context` and `subject`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "context" => Ok(Field::Context),
                            "subject" => Ok(Field::Subject),
                            "customData" => Ok(Field::CustomData),
                            "customDataContentType" => Ok(Field::CustomDataContentType),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
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
                let subject = Subject::from_json(&context.r#type, subject_json.unwrap());
                //let subject = subject.ok_or_else(|| de::Error::missing_field("subject"))?;
                Ok(CDEvent {
                    context,
                    subject,
                    custom_data,
                    custom_data_content_type,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["context", "subject"];
        deserializer.deserialize_struct("CDEvent", FIELDS, CDEventVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_json_diff::assert_json_eq;
    use rstest::rstest;
    use std::fs;
    use std::path::PathBuf;

    #[rstest]
    fn for_each_example(#[files("../cdevents-spec/examples/*.json")] path: PathBuf) {
        let example_json: serde_json::Value = serde_json::from_str(
            fs::read_to_string(&path)
                .expect("to read file as string")
                .as_str(),
        )
        .expect("to parse as json");
        dbg!(&example_json);
        let cdevent: CDEvent =
            serde_json::from_value(example_json.clone()).expect("to parse as cdevent");
        // let cdevent: CDEvent =
        //     deserialize::<CDEvent, serde_json::Value, JsonError>(example_json.clone())
        //         .expect("to parse as cdevent");

        dbg!(&cdevent);
        let cdevent_json = serde_json::to_value(cdevent).expect("to convert into json");
        dbg!(&cdevent_json);
        assert_json_eq!(example_json, cdevent_json);
    }
}
