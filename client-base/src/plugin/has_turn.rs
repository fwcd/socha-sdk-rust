/// Indicates that the value has a turn.
pub trait HasTurn {
    /// Fetches the turn.
    fn turn(&self) -> u32;
}
