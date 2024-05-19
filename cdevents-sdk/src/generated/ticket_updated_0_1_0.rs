// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

#[cfg(feature = "testkit")] use proptest_derive::Arbitrary;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "assignees", default, skip_serializing_if = "Option::is_none",)]
    pub assignees: Option<Vec<String>>,
    #[serde(rename = "creator", default, skip_serializing_if = "Option::is_none",)]
    pub creator: Option<crate::NonEmptyString>,
    #[serde(rename = "group", default, skip_serializing_if = "Option::is_none",)]
    pub group: Option<String>,
    #[serde(rename = "labels", default, skip_serializing_if = "Option::is_none",)]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "milestone", default, skip_serializing_if = "Option::is_none",)]
    pub milestone: Option<String>,
    #[serde(rename = "priority", default, skip_serializing_if = "Option::is_none",)]
    pub priority: Option<crate::NonEmptyString>,
    #[serde(rename = "summary", default, skip_serializing_if = "Option::is_none",)]
    pub summary: Option<String>,
    #[serde(rename = "ticketType", default, skip_serializing_if = "Option::is_none",)]
    pub ticket_type: Option<crate::NonEmptyString>,
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none",)]
    pub updated_by: Option<String>,
    #[serde(rename = "uri",)]
    pub uri: crate::UriReference,
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
