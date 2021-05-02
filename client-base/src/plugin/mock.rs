use serde::{Serialize, Deserialize};
use std::{fmt, str::FromStr};

use crate::error::SCError;
use crate::util::serde_as_wrapped_value;

use super::{SCPlugin, HasPlayerColor, HasOpponent, HasTurn};

/// A mock implementation of a plugin for testing.
#[derive(Debug, PartialEq, Eq)]
pub struct MockPlugin;

/// A mock player color for testing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MockPlayerColor {
    Red,
    Blue,
}

/// A mock player for testing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockPlayer {
    #[serde(with = "serde_as_wrapped_value")]
    pub color: MockPlayerColor,
    pub display_name: String,
}

/// A mock game state for testing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MockGameState {
    #[serde(with = "serde_as_wrapped_value")]
    pub player_color: MockPlayerColor,
    pub turn: u32,
}

/// A mock move for testing.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockMove;

impl SCPlugin for MockPlugin {
    type PlayerColor = MockPlayerColor;
    type Player = MockPlayer;
    type GameState = MockGameState;
    type Move = MockMove;

    fn protocol_game_type<'a>() -> &'a str { "mock" }
}

impl FromStr for MockPlayerColor {
    type Err = SCError;

    fn from_str(s: &str) -> Result<Self, SCError> {
        match s {
            "RED" => Ok(Self::Red),
            "BLUE" => Ok(Self::Blue),
            _ => Err(SCError::Custom(format!("Invalid mock player color: {}", s))),
        }
    }
}

impl fmt::Display for MockPlayerColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => write!(f, "RED"),
            Self::Blue => write!(f, "BLUE"),
        }
    }
}

impl HasOpponent for MockPlayerColor {
    fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Red,
        }
    }
}

impl HasTurn for MockGameState {
    fn turn(&self) -> u32 {
        self.turn
    }
}

impl HasPlayerColor for MockGameState {
    type PlayerColor = MockPlayerColor;

    fn player_color(&self) -> MockPlayerColor {
        self.player_color
    }
}
