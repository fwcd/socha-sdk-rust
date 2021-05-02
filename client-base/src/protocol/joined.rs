use serde::{Serialize, Deserialize};

/// A message indicating that the client
/// has joined a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "joined", rename_all = "camelCase")]
pub struct Joined {
    pub room_id: String
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    #[test]
    fn test_serialization() {
        assert_eq!(
            to_string(&super::Joined { room_id: "42".to_owned() }).unwrap().as_str(),
            r#"<joined roomId="42"/>"#
        );
    }
}
