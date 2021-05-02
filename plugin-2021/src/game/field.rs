use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_str;
use super::{Color, Vec2};

/// A field on the board holding a color.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field {
    // Unpacked position
    x: i32,
    y: i32,
    #[serde(with = "serde_as_str")]
    pub content: Color
}

impl Field {
    pub fn new(position: Vec2, content: Color) -> Self {
        Self { x: position.x, y: position.y, content }
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
