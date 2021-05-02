use serde::{Serialize, Deserialize};

// TODO: Serialize may not serialize correctly to an attribute yet
// due to https://github.com/tafia/quick-xml/issues/283

/// Determines the cause of a game score.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScoreCause {
    Regular,
    Left,
    RuleViolation,
    SoftTimeout,
    HardTimeout,
    Unknown
}
