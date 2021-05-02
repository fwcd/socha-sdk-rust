use serde::{Serialize, Deserialize};
use super::{PieceType, PlayerColor};

/// A game piece.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Piece {
    pub owner: PlayerColor,
    #[serde(rename = "type")]
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(owner: PlayerColor, piece_type: PieceType) -> Self {
        Self { owner, piece_type }
    }
}
