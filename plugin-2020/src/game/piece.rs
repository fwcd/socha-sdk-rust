use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_str;
use super::{PieceType, PlayerColor};

/// A game piece.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Piece {
    #[serde(with = "serde_as_str")]
    pub owner: PlayerColor,
    #[serde(rename = "type")]
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(owner: PlayerColor, piece_type: PieceType) -> Self {
        Self { owner, piece_type }
    }
}
