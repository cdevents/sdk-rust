// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

#[cfg(feature = "testkit")] use proptest_derive::Arbitrary;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "environment",)]
    pub environment: ContentEnvironment,
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none",)]
    pub test_suite: Option<ContentTestSuite>,
    #[serde(rename = "trigger", default, skip_serializing_if = "Option::is_none",)]
    pub trigger: Option<ContentTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct ContentTrigger {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none",)]
    pub ty: Option<ContentTriggerType>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none",)]
    pub uri: Option<crate::Uri>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct ContentTestSuite {
    #[serde(rename = "id",)]
    pub id: crate::Id,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none",)]
    pub name: Option<String>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none",)]
    pub uri: Option<crate::Uri>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none",)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct ContentEnvironment {
    #[serde(rename = "id",)]
    pub id: crate::Id,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<crate::UriReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
pub enum ContentTriggerType {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "pipeline")]
    Pipeline,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "schedule")]
    Schedule,
    #[serde(rename = "other")]
    Other,
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        #[cfg(feature = "testkit")]
        fn arbitraries_are_json_valid(s in any::<Content>()) {
            let json_str = serde_json::to_string(&s).unwrap();
            let actual = serde_json::from_str::<Content>(&json_str).unwrap();
            assert_eq!(s, actual);
        }
    }
}
