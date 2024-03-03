// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

#[cfg(feature = "testkit")] use proptest_derive::Arbitrary;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "errors", default, skip_serializing_if = "Option::is_none",)]
    pub errors: Option<String>,
    #[serde(rename = "outcome", default, skip_serializing_if = "Option::is_none",)]
    pub outcome: Option<String>,
    #[serde(rename = "pipelineRun", default, skip_serializing_if = "Option::is_none",)]
    pub pipeline_run: Option<ContentPipelineRun>,
    #[serde(rename = "taskName", default, skip_serializing_if = "Option::is_none",)]
    pub task_name: Option<String>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none",)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "testkit", derive(Arbitrary))]
#[serde(deny_unknown_fields)]
pub struct ContentPipelineRun {
    #[serde(rename = "id",)]
    pub id: crate::Id,
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