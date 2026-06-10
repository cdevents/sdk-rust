// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

#[cfg(feature = "testkit")] use proptest_derive::Arbitrary;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "approvalDetailsUrl",)]
    pub approval_details_url: crate::NonEmptyString,
    #[serde(rename = "decision",)]
    pub decision: ContentDecision,
    #[serde(rename = "resourceTargetUrl",)]
    pub resource_target_url: crate::NonEmptyString,
    #[serde(rename = "responder",)]
    pub responder: crate::Uri,
    #[serde(rename = "status",)]
    pub status: ContentStatu,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
pub enum ContentDecision {
    #[serde(rename = "escalated")]
    Escalated,
    #[serde(rename = "delegate")]
    Delegate,
    #[serde(rename = "deferred")]
    Deferred,
    #[serde(rename = "requestChanges")]
    RequestChange,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
pub enum ContentStatu {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
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
