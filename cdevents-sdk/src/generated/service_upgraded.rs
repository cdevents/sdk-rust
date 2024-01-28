// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

#[cfg(feature = "testkit")] use proptest_derive::Arbitrary;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "artifactId",)]
    pub artifact_id: String,
    #[serde(rename = "environment",)]
    pub environment: ContentEnvironment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct ContentEnvironment {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none",)]
    pub source: Option<crate::UriReference>,
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
