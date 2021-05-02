use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_str;
use super::PlayerColor;

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(with = "serde_as_str")]
    pub color: PlayerColor,
    pub display_name: String
}
