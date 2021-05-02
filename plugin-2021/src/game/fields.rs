use std::iter::FromIterator;

use serde::{Serialize, Deserialize};

use super::Field;

/// Nested list of fields. Needed due to the way XML
/// is serialized here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fields {
    #[serde(rename = "field")]
    fields: Vec<Field>
}

impl Fields {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    #[inline]
    pub fn push(&mut self, field: Field) {
        self.fields.push(field)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item=&Field> {
        self.fields.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Field> {
        self.fields.iter_mut()
    }
}

impl FromIterator<Field> for Fields {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = Field> {
        Self { fields: Vec::from_iter(iter) }
    }
}
