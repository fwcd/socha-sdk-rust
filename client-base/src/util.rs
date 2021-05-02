use serde::{Serialize, Deserialize};
use crate::error::SCError;

/// Creates a new HashMap using a literal-like syntax. It automatically
/// performs `Into` conversions for convenience.
/// 
/// Source: https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
#[macro_export]
macro_rules! hashmap {
    [ $($key:expr => $value:expr),* ] => {{
        let mut m = ::std::collections::HashMap::new();
        $(
            m.insert($key.into(), $value.into());
        )+
        m
    }}
}

/// Creates a new HashSet using a literal-like syntax.
#[macro_export]
macro_rules! hashset {
    [ $($value:expr),* ] => {{
        let mut m = ::std::collections::HashSet::new();
        $(
            m.insert($value.into());
        )+
        m
    }}
}

/// A shorthand notation for `Result<T, SCError>`.
pub type SCResult<T> = Result<T, SCError>;

/// Indicates that every variant of
/// this type has an "opponent".
pub trait HasOpponent {
    /// Fetches the opponent of this variant.
    fn opponent(self) -> Self;
}

/// A serde wrapper that uses the associated string-representation
/// e.g. of an enum that implements Display and FromStr.
/// Can be used with `#[serde(with = "serde_as_str")]`.
pub mod serde_as_str {
    use std::{str::FromStr, fmt::Debug};
    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error> where T: ToString, S: Serializer {
        serializer.serialize_str(value.to_string().as_str())
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error> where T: FromStr, T::Err: Debug, D: Deserializer<'de> {
        let raw = String::deserialize(deserializer)?;
        raw.parse().map_err(|e| D::Error::custom(format!("Could not parse string-based value: {:?}", e)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Wrap {
    #[serde(rename = "$value")]
    raw: String
}

/// A serde wrapper that uses the associated string-representation,
/// however wrapped in an additional node that uses '$value' (representing
/// the text between the XML tags).
/// Can be used with `#[serde(with = "serde_as_wrapped_value")]`.
pub mod serde_as_wrapped_value {
    use std::{str::FromStr, fmt::Debug};
    use serde::{Serialize, Deserialize, Deserializer, Serializer, de::Error};
    use super::Wrap;

    pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error> where T: ToString, S: Serializer {
        Wrap::serialize(&Wrap { raw: value.to_string() }, serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error> where T: FromStr, T::Err: Debug, D: Deserializer<'de> {
        let raw = Wrap::deserialize(deserializer)?.raw;
        raw.parse().map_err(|e| D::Error::custom(format!("Could not parse string-based value: {:?}", e)))
    }
}

/// A serde wrapper that uses the associated string-representation,
/// however wrapped in an additional node that uses '$value' (representing
/// the text between the XML tags) and wrapped in a vec.
/// Can be used with `#[serde(with = "serde_as_wrapped_value_vec")]`.
pub mod serde_as_wrapped_value_vec {
    use std::{str::FromStr, fmt::Debug};
    use serde::{Serialize, Deserialize, Deserializer, Serializer, de::Error};
    use super::Wrap;

    pub fn serialize<S, T>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error> where T: ToString, S: Serializer {
        Vec::serialize(&value.into_iter().map(|x| Wrap { raw: x.to_string() }).collect::<Vec<_>>(), serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error> where T: FromStr, T::Err: Debug, D: Deserializer<'de> {
        Vec::deserialize(deserializer)?
            .into_iter()
            .map(|raw: String| raw.parse().map_err(|e| D::Error::custom(format!("Could not parse string-based value: {:?}", e))))
            .collect::<Result<Vec<_>, _>>()
    }
}

/// A serde wrapper that uses the associated string-representation,
/// however wrapped in an additional node that uses '$value' (representing
/// the text between the XML tags) and wrapped in a hash set.
/// Can be used with `#[serde(with = "serde_as_wrapped_value_set")]`.
pub mod serde_as_wrapped_value_set {
    use std::{collections::HashSet, fmt::Debug, str::FromStr, hash::Hash};
    use serde::{Serialize, Deserialize, Deserializer, Serializer, de::Error};
    use super::Wrap;

    pub fn serialize<S, T>(value: &HashSet<T>, serializer: S) -> Result<S::Ok, S::Error> where T: ToString, S: Serializer {
        HashSet::serialize(&value.into_iter().map(|x| Wrap { raw: x.to_string() }).collect::<HashSet<_>>(), serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<HashSet<T>, D::Error> where T: FromStr + Eq + Hash, T::Err: Debug, D: Deserializer<'de> {
        HashSet::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|raw| raw.parse().map_err(|e| D::Error::custom(format!("Could not parse string-based value: {:?}", e))))
            .collect::<Result<HashSet<_>, _>>()
    }
}
