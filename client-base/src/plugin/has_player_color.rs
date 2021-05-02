/// Indicates that the value has an "associated" player color.
/// The plugin-specific `GameState` should return the current player
/// color when implementing this trait.
pub trait HasPlayerColor {
    type PlayerColor;

    /// Fetches the associated player color.
    fn player_color(&self) -> Self::PlayerColor;
}
