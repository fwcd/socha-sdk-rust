use std::{collections::HashSet, iter::FromIterator};
use socha_client_base::util::serde_as_wrapped_value_set;

use serde::{Serialize, Deserialize};

use super::PieceShape;

/// Nested list of piece shapes. Needed due to the way XML
/// is serialized here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PieceShapes {
    #[serde(rename = "shape", with = "serde_as_wrapped_value_set")]
    shapes: HashSet<PieceShape>
}

impl PieceShapes {
    pub fn new() -> Self {
        Self { shapes: HashSet::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    #[inline]
    pub fn remove(&mut self, shape: &PieceShape) -> bool {
        self.shapes.remove(shape)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&PieceShape> {
        self.shapes.iter()
    }
}

impl FromIterator<PieceShape> for PieceShapes {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = PieceShape> {
        Self { shapes: HashSet::from_iter(iter) }
    }
}
