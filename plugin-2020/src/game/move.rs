use serde::{Serialize, Deserialize};
use super::{Piece, Field};

/// A transition between two game states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "data", tag = "class")]
pub enum Move {
    #[serde(rename = "setmove")]
    SetMove { piece: Piece, destination: Field },
    #[serde(rename = "dragmove")]
    DragMove { start: Field, destination: Field },
}
