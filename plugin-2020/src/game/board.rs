use std::{cmp::max, cmp::min, collections::HashSet, collections::VecDeque, convert::TryFrom, fmt};

use arrayvec::ArrayVec;
use itertools::Itertools;
use log::trace;
use socha_client_base::util::HasOpponent;
use serde::{Deserialize, Serialize};

use crate::util::AxialCoords;

use super::{Field, Fields, Piece, PieceType, PlayerColor};

/// The game board which is a symmetric hex grid with
/// a side length of 6 fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Board {
    fields: Fields
}

impl Board {
    /// Creates a new board.
    pub fn new() -> Self {
        Self { fields: Fields::new() }
    }
    
    /// Creates a new hexagonal board.
    pub fn from_radius(radius: usize) -> Self {
        let outer = i32::try_from(radius).expect("Radius is too large to fit in a 32-bit (signed) int");
        let inner = outer - 1;
        let all_coords = ((-inner)..=inner)
            .flat_map(|y| (max(-(inner + y), -inner)..=min(inner - y, inner))
                .map(move |x| AxialCoords::new(x, y)));
        
        let mut fields = Fields::new();
        for coords in all_coords {
            fields.insert(coords);
            trace!("Filling up field at {}", coords);
        }
        
        let board = Self { fields };
        trace!("Created board with occupied fields {:?}", board.occupied_fields().collect::<Vec<_>>());
        board
    }

    /// Fetches a reference to the field at the given
    /// coordinates. The coordinates can be of and type
    /// (e.g. axial/cube) as long as they are convertible
    /// to axial coordinates.
    #[inline]
    pub fn field(&self, coords: impl Into<AxialCoords>) -> Option<&Field> {
        self.fields.get(coords.into())
    }
    
    /// Mutably borrows a field.
    pub fn field_mut(&mut self, coords: impl Into<AxialCoords>) -> Option<&mut Field> {
        self.fields.get_mut(coords.into())
    }
    
    /// Tests whether a given position is occupied.
    pub fn is_occupied(&self, coords: impl Into<AxialCoords>) -> bool {
        self.field(coords).map(|f| f.is_occupied()).unwrap_or(true)
    }
    
    /// Fetches all fields owned by the given color.
    pub fn fields_owned_by(&self, color: PlayerColor) -> impl Iterator<Item=&Field> {
        self.fields().filter(move |f| f.is_owned_by(color))
    }
    
    /// Fetches all empty fields.
    pub fn empty_fields(&self) -> impl Iterator<Item=&Field> {
        self.fields().filter(|f| f.is_empty())
    }
    
    /// Fetches all occupied fields.
    pub fn occupied_fields(&self) -> impl Iterator<Item=&Field> {
        self.fields().filter(|f| f.is_occupied())
    }
    
    /// Fetches empty fields connected to the swarm.
    pub fn swarm_boundary(&self) -> impl Iterator<Item=&Field> {
        self.fields().filter(|f| f.is_occupied())
            .flat_map(move |f| self.empty_neighbors(f.axial_coords()))
    }
    
    /// Fetches all fields.
    #[inline]
    pub fn fields(&self) -> impl Iterator<Item=&Field> {
        self.fields.iter()
    }
    
    /// Tests whether the board contains the given coordinate.
    #[inline]
    pub fn contains_coords(&self, coords: impl Into<AxialCoords>) -> bool {
        self.fields.contains(coords.into())
    }
    
    /// Tests whether the board has any pieces.
    pub fn has_pieces(&self) -> bool {
        self.fields().any(|f| f.has_pieces())
    }
    
    /// Fetches the (existing) neighbor fields on the board.
    #[inline]
    pub fn neighbors<'a>(&'a self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=&Field> + 'a {
        coords.into().coord_neighbors().into_iter().filter_map(move |c| self.field(c))
    }
    
    /// Fetches the unoccupied neighbor fields.
    pub fn empty_neighbors(&self, coords: impl Into<AxialCoords>) -> impl Iterator<Item=&Field> {
        self.neighbors(coords).filter(|f| f.is_empty())
    }
    
    /// Tests whether the bee of the given color has been placed.
    pub fn has_placed_bee(&self, color: PlayerColor) -> bool {
        let bee = Piece { piece_type: PieceType::Bee, owner: color };
        self.fields().flat_map(|f| f.pieces()).any(|&p| p == bee)
    }
    
    /// Tests whether the field at the given coordinates is next to
    /// a given color.
    pub fn is_next_to(&self, color: PlayerColor, coords: impl Into<AxialCoords>) -> bool {
        self.neighbors(coords).any(|f| f.is_owned_by(color))
    }
    
    /// Tests whether the field at the given coordinates is adjacent
    /// to a field.
    pub fn is_next_to_piece(&self, coords: impl Into<AxialCoords>) -> bool {
        self.neighbors(coords).any(|f| f.has_pieces())
    }
    
    /// Fetches the possible destinations for a SetMove.
    pub fn possible_set_move_destinations<'a>(&'a self, color: PlayerColor) -> impl Iterator<Item=AxialCoords> + 'a {
        let opponent = color.opponent();

        trace!("Looking for SetMove destinations on board...");
        trace!("Fields owned by {:?}: {:#?}", color, self.fields_owned_by(color).collect::<Vec<_>>());
        trace!("Fields owned by {:?} (opponent): {:#?}", opponent, self.fields_owned_by(opponent).collect::<Vec<_>>());

        self.fields_owned_by(color)
            .flat_map(move |f| self.empty_neighbors(f.axial_coords()))
            .unique()
            .filter_map(move |f| if self.is_next_to(opponent, f.axial_coords()) { None } else {
                trace!("SetMove destination {} does not touch an opponent's ({:?}'s) piece, neighbors: {:#?}", f.axial_coords(), opponent, self.neighbors(f.axial_coords()).collect::<Vec<_>>());
                Some(f.coords())
            })
    }
    
    /// Performs a depth-first search on the board's non-empty fields
    /// starting at the given coordinates and removing visited
    /// locations from the set.
    fn dfs_swarm(&self, coords: AxialCoords, unvisited: &mut HashSet<AxialCoords>) {
        if self.field(coords).filter(|f| f.has_pieces()).is_some() {
            unvisited.remove(&coords);
            for neighbor in self.neighbors(coords) {
                let neighbor_coords = neighbor.coords();
                if unvisited.contains(&neighbor_coords) {
                    self.dfs_swarm(neighbor_coords, unvisited)
                }
            }
        }
    }
    
    /// Tests whether a field satisfying the search condition can be
    /// reached by breadth-first searching the accessible fields.
    fn bfs_accessible(&self, start: AxialCoords, search_condition: impl Fn(&Field) -> bool) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start);
        
        while let Some(coords) = queue.pop_front() {
            visited.insert(coords);

            if let Some(field) = self.field(coords) {
                if search_condition(field) {
                    return true;
                } else {
                    queue.extend(self.accessible_neighbors_except(Some(start), coords).filter_map(|f| if !visited.contains(&f.coords()) { Some(f.axial_coords()) } else { None }));
                }
            }
        }

        false
    }
    
    /// Tests whether the given field can be reached in 3 moves
    /// by breadth-first searching the accessible fields.
    pub fn bfs_reachable_in_3_steps(&self, start: AxialCoords, destination: AxialCoords) -> bool {
        let mut paths_queue: VecDeque<ArrayVec<AxialCoords, 3>> = VecDeque::new();
        paths_queue.push_back({
            let mut path = ArrayVec::new();
            path.push(start);
            path
        });

        while let Some(path) = paths_queue.pop_front() {
            let mut neighbors = self.accessible_neighbors_except(Some(start), path.last().cloned().unwrap()).filter(|f| !path.contains(&f.axial_coords()));
            if path.len() < 3 {
                paths_queue.extend(neighbors.map(|f| {
                    let mut next_path = path.clone();
                    next_path.push(f.axial_coords());
                    next_path
                }));
            } else if neighbors.any(|f| f.axial_coords() == destination) {
                return true;
            }
        }

        false
    }
    
    /// Finds the intersection between `a`'s and `b`'s neighbors,
    /// optionally given an exception whose field won't be included
    /// if it contains exactly one piece.
    pub fn shared_neighbors(&self, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>, exception: Option<AxialCoords>) -> Vec<&Field> {
        let a_neighbors: HashSet<_> = self.neighbors(a).collect();
        let b_neighbors: HashSet<_> = self.neighbors(b).collect();
        a_neighbors.intersection(&b_neighbors)
            .filter(|f| f.pieces().len() != 1 || exception == Some(f.axial_coords()))
            .cloned().collect()
    }
    
    /// Tests whether a move between the given two
    /// locations is possible, optionally given an
    /// exception.
    pub fn can_move_between_except(&self, exception: Option<AxialCoords>, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>) -> bool {
        let shared = self.shared_neighbors(a, b, exception);
        (shared.len() == 1 || shared.iter().any(|f| f.is_empty())) && shared.iter().any(|f| f.has_pieces())
    }
    
    /// Tests whether a move between the given two
    /// locations is possible.
    pub fn can_move_between(&self, a: impl Into<AxialCoords>, b: impl Into<AxialCoords>) -> bool {
        self.can_move_between_except(None, a, b)
    }
    
    /// Finds the accessible neighbors, optionally except an ignored field.
    pub fn accessible_neighbors_except<'a>(&'a self, exception: Option<AxialCoords>, coords: impl Into<AxialCoords> + Copy + 'a) -> impl Iterator<Item=&Field> + 'a {
        self.neighbors(coords).filter(move |f| f.is_empty() && self.can_move_between_except(exception, coords, f.axial_coords()))
    }
    
    /// Finds the accessible neighbors.
    pub fn accessible_neighbors<'a>(&'a self, coords: impl Into<AxialCoords> + Copy + 'a) -> impl Iterator<Item=&Field> + 'a {
        self.neighbors(coords).filter(move |f| f.is_empty() && self.can_move_between(coords, f.axial_coords()))
    }
    
    /// Tests whether two coordinates are connected by a path
    /// along the swarm's boundary.
    pub fn connected_by_boundary_path(&self, start_coords: impl Into<AxialCoords>, destination_coords: impl Into<AxialCoords>) -> bool {
        let start = start_coords.into();
        let destination = destination_coords.into();
        self.bfs_accessible(start, |f| f.axial_coords() == destination)
    }
    
    /// Performs a depth-first search on the board at the given
    /// position to test whether the swarm is connected.
    pub fn is_swarm_connected(&self) -> bool {
        let mut unvisited = self.fields.iter()
            .filter_map(|f| if f.has_pieces() { Some(f.axial_coords()) } else { None })
            .collect::<HashSet<AxialCoords>>();

        if let Some(start) = unvisited.iter().next() {
            self.dfs_swarm(*start, &mut unvisited);
            unvisited.is_empty()
        } else {
            true // An empty swarm is connected
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x = self.fields().map(|f| f.axial_coords().x).min().ok_or(fmt::Error)?;
        let min_y = self.fields().map(|f| f.axial_coords().y).min().ok_or(fmt::Error)?;
        let max_x = self.fields().map(|f| f.axial_coords().x).max().ok_or(fmt::Error)?;
        let max_y = self.fields().map(|f| f.axial_coords().y).max().ok_or(fmt::Error)?;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(field) = self.field(AxialCoords::new(-y, -x)) {
                    write!(f, "{}", field)?;
                } else {
                    write!(f, "00")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter::once;
    use super::Board;
    use crate::game::{Field, Piece, PieceType, PlayerColor};
    use crate::util::AxialCoords;

    macro_rules! assert_unordered_eq {
        ($a:expr, $b:expr) => {
            assert_eq!(
                $a.into_iter().collect::<::std::collections::HashSet<_>>(),
                $b.into_iter().collect::<::std::collections::HashSet<_>>()
            )
        };
    }

    #[test]
    fn test_filled_grid() {
        let mut board = Board::from_radius(3);
        assert_eq!(board.fields().count(), 19);

        board.field_mut(AxialCoords::new(0, 0)).unwrap().push(Piece::new(PlayerColor::Red, PieceType::Grasshopper));
        board.field_mut(AxialCoords::new(0, 1)).unwrap().push(Piece::new(PlayerColor::Red, PieceType::Bee));
        board.field_mut(AxialCoords::new(1, 0)).unwrap().push(Piece::new(PlayerColor::Blue, PieceType::Ant));
        board.field_mut(AxialCoords::new(2, -1)).unwrap().push(Piece::new(PlayerColor::Blue, PieceType::Ant));
        assert!(board.has_pieces());

        assert_unordered_eq!(board.fields_owned_by(PlayerColor::Red).cloned(), vec![
            Field::new(AxialCoords::new(0, 0), once(Piece {
                piece_type: PieceType::Grasshopper,
                owner: PlayerColor::Red
            }), false),
            Field::new(AxialCoords::new(0, 1), once(Piece {
                piece_type: PieceType::Bee,
                owner: PlayerColor::Red
            }), false)
        ]);
        assert_unordered_eq!(board.fields_owned_by(PlayerColor::Blue).cloned(), vec![
            Field::new(AxialCoords::new(1, 0), once(Piece {
                piece_type: PieceType::Ant,
                owner: PlayerColor::Blue
            }), false),
            Field::new(AxialCoords::new(2, -1), once(Piece {
                piece_type: PieceType::Ant,
                owner: PlayerColor::Blue
            }), false)
        ]);
    }
}
