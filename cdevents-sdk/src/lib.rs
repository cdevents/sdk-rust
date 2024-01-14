// TODO remove unwrap(), expect(...)
// TODO reduce clone()
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
                    Subject::from_json(&context.r#type, subject_json).map_err(de::Error::custom)?;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subject {
    #[serde(rename = "content")]
    pub content: Content,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
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
            source: json["source"].as_str().map(|s| s.to_string()),
            content: Content::from_json(ty, json["content"].clone())?,
        })
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
        dbg!(&cdevent);
        let cdevent_json = serde_json::to_value(cdevent).expect("to convert into json");
        dbg!(&cdevent_json);
        assert_json_eq!(example_json, cdevent_json);
    }
}
