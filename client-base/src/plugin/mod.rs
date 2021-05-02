mod has_player_color;
mod has_turn;
mod mock;

pub use has_player_color::*;
pub use has_turn::*;
pub use mock::*;

use std::str::FromStr;
use std::fmt::Debug;
use crate::util::HasOpponent;
use crate::error::SCError;

/// A "type family" trait that defines types
/// which represent various parts of a game.
pub trait SCPlugin: Debug {
    type PlayerColor: Copy + Debug + Eq + HasOpponent + FromStr<Err=SCError>;
    type Player: Clone + Debug + Eq;
    type GameState: Clone + Debug + Eq + HasPlayerColor<PlayerColor=Self::PlayerColor> + HasTurn;
    type Move: Clone + Debug + Eq;
    
    /// Fetches the 'gameType' used during
    /// the protocol handshake.
    fn protocol_game_type<'a>() -> &'a str;
}
