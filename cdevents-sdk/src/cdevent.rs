use crate::{Context, Subject};
use serde::{
    de::{self, Deserializer, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Debug, Clone, Serialize)]
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
