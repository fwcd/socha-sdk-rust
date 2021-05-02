use serde::{Serialize, Deserialize};
use super::ScoreCause;

/// The score of a game player.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerScore {
    pub cause: ScoreCause,
    pub reason: String
}
