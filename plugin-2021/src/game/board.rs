use serde::{Serialize, Deserialize};

use super::{CORNERS, Color, Vec2, Corner, Field, Fields, Piece};

pub const BOARD_SIZE: usize = 20;

/// The game board is a 20x20 grid of fields with colors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    fields: Fields
}

impl Board {
    /// Creates an empty board.
    pub fn new() -> Self {
        Self { fields: Fields::new() }
    }

    /// Creates a new board from the given fields.
    pub fn from_fields(fields: impl IntoIterator<Item=Field>) -> Self {
        Self { fields: fields.into_iter().collect() }
    }

    /// Fetches the number of occupied fields.
    pub fn count_obstructed(&self) -> usize {
        self.fields.iter().filter(|f| f.content != Color::None).count()
    }

    /// Checks whether the given coordinates are in the board's bounds.
    pub fn is_in_bounds(coordinates: Vec2) -> bool {
           coordinates.x >= 0
        && coordinates.y >= 0
        && coordinates.x < BOARD_SIZE as i32
        && coordinates.y < BOARD_SIZE as i32
    }

    /// Fetches the board's corners.
    pub fn corner_positions() -> impl Iterator<Item=Vec2> {
        CORNERS.iter().map(|&c| Self::corner_position(c)).collect::<Vec<_>>().into_iter()
    }

    /// Fetches the position of a corner.
    pub fn corner_position(corner: Corner) -> Vec2 {
        match corner {
            Corner::TopLeft => Vec2::new(0, 0),
            Corner::BottomLeft => Vec2::new(0, BOARD_SIZE as i32 - 1),
            Corner::TopRight => Vec2::new(BOARD_SIZE as i32 - 1, 0),
            Corner::BottomRight => Vec2::new(BOARD_SIZE as i32 - 1, BOARD_SIZE as i32 - 1)
        }
    }

    /// Aligns a position to a corner.
    pub fn align(area: Vec2, corner: Corner) -> Vec2 {
        let position = Self::corner_position(corner);
        match corner {
            Corner::TopLeft => position,
            Corner::TopRight => Vec2::new(position.x - area.x, position.y),
            Corner::BottomLeft => Vec2::new(position.x, position.y - area.y),
            Corner::BottomRight => position - area
        }
    }

    /// Checks whether a coordinate is on a corner.
    pub fn is_on_corner(position: Vec2) -> bool {
        Self::corner_positions().any(|p| p == position)
    }

    /// Fetches the color at the given position.
    pub fn get(&self, position: Vec2) -> Color {
        // TODO: This is very inefficient and would be much better handled using a matrix
        self.fields.iter().find(|f| f.position() == position).map(|f| f.content).unwrap_or_default()
    }

    /// Places the color at the given position.
    pub fn set(&mut self, position: Vec2, color: Color) {
        // TODO: This is very inefficient and would be much better handled using a matrix
        let found = self.fields.iter_mut().find(|f| f.position() == position);
        match found {
            Some(field) => field.content = color,
            None => self.fields.push(Field::new(position, color))
        }
    }

    /// Places the given piece on the board WITH NO ADDITIONAL CHECKS.
    pub fn place(&mut self, piece: &Piece) {
        for position in piece.coordinates() {
            self.set(position, piece.color);
        }
    }

    /// Checks whether the given position is obstructed.
    pub fn is_obstructed(&self, position: Vec2) -> bool {
        self.fields.iter().any(|f| f.position() == position && f.content != Color::None)
    }

    /// Checks whether the position touches another border of same color.
    pub fn borders_on_color(&self, position: Vec2, color: Color) -> bool {
        [
            Vec2::new(1, 0),
            Vec2::new(0, 1),
            Vec2::new(-1, 0),
            Vec2::new(0, -1)
        ].iter().any(|&o| self.get(position + o) == color)
    }

    /// Checks whether the position touches another corner of same color.
    pub fn corners_on_color(&self, position: Vec2, color: Color) -> bool {
        [
            Vec2::new(1, 1),
            Vec2::new(1, 1),
            Vec2::new(-1, 1),
            Vec2::new(1, -1)
        ].iter().any(|&o| self.get(position + o) == color)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use quick_xml::de::from_str;
    use crate::game::{Color, Field, Vec2};

    use super::Board;

    #[test]
    fn test_deserialization() {
        let raw = indoc! {r#"
            <board>
                <fields>
                    <field x="0" y="0" content="BLUE" />
                    <field x="0" y="1" content="RED" />
                </fields>
            </board>
        "#};
        let board: Board = from_str(raw).unwrap();
        assert_eq!(
            board,
            // Ideally we would want to .into_iter() here,
            // however into_iter for fixed-size arrays is
            // first coming in Rust 2021
            // See: https://github.com/rust-lang/rust/issues/25725
            Board::from_fields([
                Field::new(Vec2::new(0, 0), Color::Blue),
                Field::new(Vec2::new(0, 1), Color::Red),
            ].iter().cloned())
        )
    }
}
