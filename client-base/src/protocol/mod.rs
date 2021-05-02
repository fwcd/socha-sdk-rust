//! The data structures used by the XML protocol.

mod packet;
mod event;
mod game_result;
mod joined;
mod left;
mod close;
mod player_score;
mod room;
mod score_aggregation;
mod score_cause;
mod score_definition;
mod score_fragment;

pub use packet::*;
pub use event::*;
pub use game_result::*;
pub use joined::*;
pub use left::*;
pub use close::*;
pub use event::*;
pub use player_score::*;
pub use room::*;
pub use score_definition::*;
pub use score_fragment::*;
pub use score_aggregation::*;
pub use score_cause::*;
