use serde::{Serialize, Deserialize};
use super::ScoreAggregation;
use crate::util::serde_as_wrapped_value;

/// A single score fragment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreFragment {
    pub name: String,
    #[serde(with = "serde_as_wrapped_value")]
    pub aggregation: ScoreAggregation,
    pub relevant_for_ranking: bool,
}
