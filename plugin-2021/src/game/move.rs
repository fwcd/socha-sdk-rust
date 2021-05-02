use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_wrapped_value;

use super::{Color, Piece};

/// A move in the game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "data", tag = "class")]
pub enum Move {
    /// A move that skips a round.
    #[serde(rename = "sc.plugin2021.SkipMove")]
    Skip {
        #[serde(with = "serde_as_wrapped_value")]
        color: Color
    },
    /// A move that places an own, not yet placed piece.
    #[serde(rename = "sc.plugin2021.SetMove")]
    Set {
        piece: Piece
    }
}

impl Move {
    pub fn color(&self) -> Color {
        match self {
            Self::Skip { color } => *color,
            Self::Set { piece } => piece.color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Move;
    use crate::game::{Color, PIECE_SHAPES_BY_NAME, Piece, Rotation, Vec2};
    use quick_xml::se::to_string;

    #[test]
    fn test_serialization() {
        let set_move = Move::Set {
            piece: Piece {
                kind: PIECE_SHAPES_BY_NAME["TRIO_L"].clone(),
                rotation: Rotation::Mirror,
                is_flipped: true,
                color: Color::Blue,
                position: Vec2::new(2, 3),
            }
        };
        assert_eq!(
            to_string(&set_move).unwrap().as_str(),
            r#"<data class="sc.plugin2021.SetMove"><piece kind="TRIO_L" rotation="MIRROR" isFlipped="true" color="BLUE"><position x="2" y="3"/></piece></data>"#
        );

        let skip_move = Move::Skip {
            color: Color::Red
        };
        assert_eq!(
            to_string(&skip_move).unwrap().as_str(),
            r#"<data class="sc.plugin2021.SkipMove"><color>RED</color></data>"#
        )
    }
}
