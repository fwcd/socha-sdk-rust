use std::{iter::FromIterator, ops::Index};

use serde::{Serialize, Deserialize};
use socha_client_base::util::serde_as_wrapped_value_vec;

use super::Color;

/// Nested list of colors. Needed due to the way XML is serialized here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Colors {
    #[serde(rename = "color", with = "serde_as_wrapped_value_vec")]
    colors: Vec<Color>
}

impl Colors {
    pub fn new() -> Self {
        Self { colors: Vec::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&Color> {
        self.colors.iter()
    }
}

impl Index<usize> for Colors {
    type Output = Color;

    fn index(&self, index: usize) -> &Color {
        &self.colors[index]
    }
}

impl FromIterator<Color> for Colors {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Color> {
        Self { colors: Vec::from_iter(iter) }
    }
}
