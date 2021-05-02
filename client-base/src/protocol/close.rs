use serde::{Serialize, Deserialize};

/// A message indicating that the connection
/// has been closed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "close")]
pub struct Close;
