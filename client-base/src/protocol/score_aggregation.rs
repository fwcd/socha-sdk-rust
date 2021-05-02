use std::{fmt, str::FromStr};

use crate::error::SCError;

/// Determines how scores should be aggregated (e.g. summed up or averaged over).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScoreAggregation {
    Sum,
    Average
}

impl fmt::Display for ScoreAggregation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sum => write!(f, "SUM"),
            Self::Average => write!(f, "AVERAGE"),
        }
    }
}

impl FromStr for ScoreAggregation {
    type Err = SCError;

    fn from_str(s: &str) -> Result<Self, SCError> {
        match s {
            "SUM" => Ok(Self::Sum),
            "AVERAGE" => Ok(Self::Average),
            _ => Err(format!("Could not parse aggregation: {}", s).into())
        }
    }
}
