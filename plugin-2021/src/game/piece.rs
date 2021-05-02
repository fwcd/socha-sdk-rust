use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_str;

use super::{Color, Vec2, PieceShape, Rotation};

/// A game piece with color, position and transformed form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Piece {
    /// The piece's untransformed shape
    #[serde(with = "serde_as_str")]
    pub kind: PieceShape,
    /// How far the piece has been rotated
    #[serde(with = "serde_as_str")]
    pub rotation: Rotation,
    /// Whether the piece has been mirrored along the y-axis
    pub is_flipped: bool,
    /// The piece's color
    #[serde(with = "serde_as_str")]
    pub color: Color,
    /// The top left corner of the piece's rectangular bounding box
    pub position: Vec2
}

impl Piece {
    /// Fetches the piece's actual (transformed) shape
    pub fn shape(&self) -> PieceShape {
        self.kind.transform(self.rotation, self.is_flipped)
    }

    /// Fetches the piece's actual coordinates.
    pub fn coordinates(&self) -> impl Iterator<Item=Vec2> {
        let position = self.position;
        self.shape().coordinates().map(move |c| c + position)
    }
}
