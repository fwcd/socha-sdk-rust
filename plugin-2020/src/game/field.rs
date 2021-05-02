use std::{convert::TryFrom, fmt, iter::empty};

use regex::Regex;
use serde::{Serialize, Deserialize};
use socha_client_base::{error::SCError, util::SCResult};
use lazy_static::lazy_static;

use super::{Piece, PieceType, PlayerColor};
use crate::util::{AxialCoords, CubeCoords, DoubledCoords};

lazy_static! {
    /// The syntax used for fields when parsing
    /// ASCII hex grid fields.
    static ref FIELD_SYNTAX: Regex = Regex::new(r"^([A-Z])([A-Z])$").unwrap();
}

/// A field on the game board.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    // Unpacked cube coords
    x: i32,
    y: i32,
    z: i32,
    /// The piece stack.
    #[serde(rename = "piece")]
    pieces: Vec<Piece>,
    is_obstructed: bool
}

impl Field {
    /// Creates a new field.
    pub fn new(coords: impl Into<CubeCoords>, pieces: impl IntoIterator<Item=Piece>, is_obstructed: bool) -> Self {
        let cube_coords = coords.into();
        Self {
            x: cube_coords.x,
            y: cube_coords.y,
            z: cube_coords.z,
            pieces: pieces.into_iter().collect(),
            is_obstructed
        }
    }

    /// Creates a new field at the given position.
    pub fn from_coords(coords: impl Into<CubeCoords>) -> Self {
        Self::new(coords, empty(), false)
    }

    /// Converts a field in a two-character notation
    /// to a field. The first character denotes the
    /// player color and the second character describes the
    /// piece type.
    /// 
    /// Obstructed fields and piece stacks are not (yet)
    /// supported.
    pub fn from_short(coords: impl Into<CubeCoords>, raw: &str) -> SCResult<Self> {
        if raw.is_empty() {
            Ok(Self::new(coords, Vec::new(), false))
        } else {
            let groups = FIELD_SYNTAX.captures(raw).ok_or_else(|| SCError::from(format!("{} does not match field syntax {}", raw, FIELD_SYNTAX.as_str())))?;
            let owner = PlayerColor::try_from(groups[1].chars().next().unwrap())?;
            let piece_type = PieceType::try_from(groups[2].chars().next().unwrap())?;
            let piece = Piece { piece_type, owner };
            Ok(Self::new(coords, vec![piece], false))
        }
    }

    /// Fetches the coordinates of the field.
    pub fn coords<C>(&self) -> C where C: From<CubeCoords> { CubeCoords::new(self.x, self.y, self.z).into() }

    /// Fetches the axial coordinates of the field.
    #[inline]
    pub fn axial_coords(&self) -> AxialCoords { self.coords() }

    /// Fetches the cube coordinates of the field.
    #[inline]
    pub fn cube_coords(&self) -> CubeCoords { self.coords() }

    /// Fetches the doubled coordinates of the field.
    #[inline]
    pub fn doubled_coords(&self) -> DoubledCoords { self.coords() }

    /// Fetches the player color "owning" the field.
    pub fn owner(&self) -> Option<PlayerColor> { self.piece().map(|p| p.owner) }
    
    /// Tests whether the field is owned by the given owner.
    #[inline]
    pub fn is_owned_by(&self, color: PlayerColor) -> bool { self.owner() == Some(color) }

    /// Tests whether the field is (directly) obstructed.
    #[inline]
    pub fn is_obstructed(&self) -> bool { self.is_obstructed }
    
    /// Tests whether the field is occupied.
    #[inline]
    pub fn is_occupied(&self) -> bool { self.is_obstructed || self.has_pieces() }
    
    /// Tests whether the field is not occupied.
    #[inline]
    pub fn is_empty(&self) -> bool { !self.is_occupied() }
    
    /// Fetches the top-most piece.
    #[inline]
    pub fn piece(&self) -> Option<Piece> { self.pieces.last().cloned() }
    
    /// Tests whether the field contains pieces.
    #[inline]
    pub fn has_pieces(&self) -> bool { !self.pieces.is_empty() }
    
    /// Fetches the piece stack.
    #[inline]
    pub fn pieces(&self) -> &Vec<Piece> { &self.pieces }
    
    /// Pushes a piece onto the piece stack.
    #[inline]
    pub fn push(&mut self, piece: Piece) { self.pieces.push(piece) }
    
    /// Pops a piece from the piece stack or
    /// returns `None` if the stack is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<Piece> { self.pieces.pop() }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(piece) = self.piece() {
            write!(f, "{}{}", char::from(piece.owner), char::from(piece.piece_type))
        } else {
            write!(f, "[]")
        }
    }
}
