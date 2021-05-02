use std::{fmt, str::FromStr};
use serde::{Serialize, Deserialize};
use crate::plugin::SCPlugin;
use super::{Room, Joined, Left, Close};

/// A generic message packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Packet<P> where P: SCPlugin {
    #[serde(bound(
        serialize = "P::Player: Serialize, P::Move: Serialize, P::PlayerColor: fmt::Display, P::GameState: Serialize",
        deserialize = "P::Player: Deserialize<'de>, P::Move: Deserialize<'de>, P::PlayerColor: FromStr, P::GameState: Deserialize<'de>"
    ))]
    Room(Room<P>),
    Joined(Joined),
    Left(Left),
    Close(Close)
}
