use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_wrapped_value;

use super::Team;

/// Metadata about a player.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    #[serde(rename = "color", with = "serde_as_wrapped_value")]
    pub team: Team,
    pub display_name: String
}
