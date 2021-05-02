use serde::{Serialize, Deserialize};
use super::ScoreFragment;

/// The definition of a score.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreDefinition {
    #[serde(rename = "fragment")]
    pub fragments: Vec<ScoreFragment>
}
