//! Defines useful structures for geometry on
//! hexagonal grids.

use arrayvec::ArrayVec;
use serde::{Serialize, Deserialize};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div};
use std::collections::HashMap;
use std::fmt;
use socha_client_base::hashmap;

/// Axial coordinates on the hex grid.
/// 
/// See https://www.redblobgames.com/grids/hexagons/#coordinates-axial
/// for a description.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AxialCoords {
    pub x: i32,
    pub y: i32
}

/// Cube coordinates on the hex grid.
/// These are used by the protocol internally.
/// 
/// See https://www.redblobgames.com/grids/hexagons/#coordinates-cube
/// for a description.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CubeCoords {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

/// Offset coordinates with a doubled vertical
/// step size on the hex grid. When converting
/// to `AxialCoords`, these coordinates are
/// interpreted with the following axes:
/// 
/// ```ignore
/// +--> x
/// |
/// v y
/// ```
/// 
/// ...and use the following axes after the
/// conversion to `AxialCoords`:
/// 
/// ```ignore
///  y ^   ^ x
///     \ /  
///      +
/// ```
/// 
/// These are especially useful when dealing with
/// ASCII hex-grids.
/// 
/// See https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
/// for a description.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DoubledCoords {
    pub x: i32,
    pub y: i32
}

/// An iterator that returns coordinates on
/// a straight line.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LineIter {
    current: CubeCoords,
    destination: CubeCoords,
    step: CubeCoords
}

pub trait LineFormable {
    /// Tests whether `self` and `rhs` form a
    /// straight line.
    fn forms_line_with(self, rhs: Self) -> bool;
    
    /// Fetches the elements _between_ `self` and `rhs`
    /// in cube coordinates.
    fn line_iter(self, rhs: Self) -> LineIter;
}

pub trait Adjacentable {
    /// Tests whether `self` and `rhs` are neighbors.
    fn is_adjacent_to(self, rhs: Self) -> bool;
}

impl AxialCoords {
    /// Creates new axial coordinates.
    #[inline]
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
    
    /// Fetches all 6 neighbors, regardless of any board
    /// boundaries.
    #[inline]
    pub fn coord_neighbors(self) -> ArrayVec<AxialCoords, 6> {
        ArrayVec::from([
            self + AxialCoords::new(0, 1),
            self + AxialCoords::new(1, 0),
            self + AxialCoords::new(1, -1),
            self + AxialCoords::new(0, -1),
            self + AxialCoords::new(-1, 0),
            self + AxialCoords::new(-1, 1)
        ])
    }
}

impl CubeCoords {
    /// Creates new (unvalidated) cube coordinates.
    #[inline]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Creates new cube coordinates if they are valid.
    #[inline]
    pub fn new_valid(x: i32, y: i32, z: i32) -> Option<Self> {
        if (x + y + z) == 0 {
            Some(CubeCoords { x, y, z })
        } else {
            None
        }
    }
}

impl DoubledCoords {
    /// Creates new doubled coordinates.
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for AxialCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for CubeCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for DoubledCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<C> LineFormable for C where C: Into<CubeCoords> {
    fn forms_line_with(self, rhs: Self) -> bool {
        let lhs_cube = self.into();
        let rhs_cube = rhs.into();
        lhs_cube.x == rhs_cube.x || lhs_cube.y == rhs_cube.y || lhs_cube.z == rhs_cube.z
    }
    
    fn line_iter(self, rhs: Self) -> LineIter {
        let lhs_cube = self.into();
        let rhs_cube = rhs.into();
        let diff = rhs_cube - lhs_cube;
        let step = CubeCoords::new(diff.x.signum(), diff.y.signum(), diff.z.signum());
        LineIter::new(lhs_cube + step, step, rhs_cube)
    }
}

impl LineIter {
    pub fn new(start: CubeCoords, step: CubeCoords, destination: CubeCoords) -> Self {
        Self { current: start, step: step, destination: destination }
    }
}

impl Iterator for LineIter {
    type Item = CubeCoords;
    
    fn next(&mut self) -> Option<CubeCoords> {
        if self.current == self.destination {
            None
        } else {
            let pos = self.current;
            self.current += self.step;
            Some(pos)
        }
    }
}

impl<C> Adjacentable for C where C: Into<AxialCoords> {
    fn is_adjacent_to(self, rhs: Self) -> bool {
        let lhs_axial = self.into();
        let rhs_axial = rhs.into();
        lhs_axial.coord_neighbors().iter().any(|&c| c == rhs_axial)
    }
}

// Operator overloads

impl Add for AxialCoords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Sub for AxialCoords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y } }
}

impl<R> Mul<R> for AxialCoords where R: Into<i32> {
    type Output = Self;
    
    fn mul(self, rhs: R) -> Self {
        let other = rhs.into();
        Self { x: self.x * other, y: self.y * other }
    }
}

impl<R> Div<R> for AxialCoords where R: Into<i32> {
    type Output = Self;
    
    fn div(self, rhs: R) -> Self {
        let other = rhs.into();
        Self { x: self.x / other, y: self.y / other }
    }
}

impl AddAssign for AxialCoords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for AxialCoords {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<R> MulAssign<R> for AxialCoords where R: Into<i32> {
    fn mul_assign(&mut self, rhs: R) {
        let r = rhs.into();
        self.x *= r;
        self.y *= r;
    }
}

impl Add for CubeCoords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.y + rhs.z } }
}

impl Sub for CubeCoords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } }
}

impl<R> Mul<R> for CubeCoords where R: Into<i32> {
    type Output = Self;
    
    fn mul(self, rhs: R) -> Self {
        let other = rhs.into();
        Self { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}

impl<R> Div<R> for CubeCoords where R: Into<i32> {
    type Output = Self;
    
    fn div(self, rhs: R) -> Self {
        let other = rhs.into();
        Self { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}

impl AddAssign for CubeCoords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for CubeCoords {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<R> MulAssign<R> for CubeCoords where R: Into<i32> {
    fn mul_assign(&mut self, rhs: R) {
        let r = rhs.into();
        self.x *= r;
        self.y *= r;
        self.z += r;
    }
}

impl Add for DoubledCoords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl Sub for DoubledCoords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y } }
}

impl<R> Mul<R> for DoubledCoords where R: Into<i32> {
    type Output = Self;
    
    fn mul(self, rhs: R) -> Self {
        let other = rhs.into();
        Self { x: self.x * other, y: self.y * other }
    }
}

impl<R> Div<R> for DoubledCoords where R: Into<i32> {
    type Output = Self;
    
    fn div(self, rhs: R) -> Self {
        let other = rhs.into();
        Self { x: self.x / other, y: self.y / other }
    }
}

impl AddAssign for DoubledCoords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for DoubledCoords {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<R> MulAssign<R> for DoubledCoords where R: Into<i32> {
    fn mul_assign(&mut self, rhs: R) {
        let r = rhs.into();
        self.x *= r;
        self.y *= r;
    }
}

// Direct conversions

impl From<CubeCoords> for AxialCoords {
    fn from(coords: CubeCoords) -> Self { Self { x: coords.x, y: coords.y } }
}

impl From<AxialCoords> for CubeCoords {
    fn from(coords: AxialCoords) -> Self { Self { x: coords.x, y: coords.y, z: -(coords.x + coords.y) } }
}

impl From<AxialCoords> for DoubledCoords {
    fn from(coords: AxialCoords) -> Self {
        Self { x: coords.x - coords.y, y: -(coords.x + coords.y) }
    }
}

impl From<DoubledCoords> for AxialCoords {
    fn from(coords: DoubledCoords) -> Self {
        Self { x: (coords.x - coords.y) / 2, y: -(coords.x + coords.y) / 2 }
    }
}

// Transitive conversions

impl From<CubeCoords> for DoubledCoords {
    fn from(coords: CubeCoords) -> Self { Self::from(AxialCoords::from(coords)) }
}

impl From<DoubledCoords> for CubeCoords {
    fn from(coords: DoubledCoords) -> Self { Self::from(AxialCoords::from(coords)) }
}

// Other conversions

impl From<CubeCoords> for HashMap<String, String> {
    fn from(coords: CubeCoords) -> Self {
        hashmap!["x" => coords.x.to_string(), "y" => coords.y.to_string(), "z" => coords.z.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use super::AxialCoords as Axial;
    use super::DoubledCoords as Doubled;

    /// Tests whether a bidirectional conversion
    /// succeeds in both directions.
    fn test_bi_conversion<A, B>(a: A, b: B) where A: From<B> + Eq + Debug + Clone, B: From<A> + Eq + Debug + Clone {
        assert_eq!(B::from(a.clone()), b.clone());
        assert_eq!(A::from(b), a);
    }

    #[test]
    fn test_doubled_axial_coords() {
        test_bi_conversion(Axial::new(0, 1), Doubled::new(-1, -1));
        test_bi_conversion(Axial::new(1, 0), Doubled::new(1, -1));
        test_bi_conversion(Axial::new(-1, 1), Doubled::new(-2, 0));
        test_bi_conversion(Axial::new(0, 0), Doubled::new(0, 0));
        test_bi_conversion(Axial::new(1, -1), Doubled::new(2, 0));
        test_bi_conversion(Axial::new(-2, 1), Doubled::new(-3, 1));
        test_bi_conversion(Axial::new(-1, 0), Doubled::new(-1, 1));
        test_bi_conversion(Axial::new(0, -1), Doubled::new(1, 1));
    }
}
