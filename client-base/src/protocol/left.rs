use serde::{Serialize, Deserialize};

/// A message indicating that the client
/// has left a room with the specified id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "left", rename_all = "camelCase")]
pub struct Left {
    pub room_id: String
}

#[cfg(test)]
mod tests {
    use quick_xml::se::to_string;

    #[test]
    fn test_serialization() {
        // See https://cau-kiel-tech-inf.github.io/socha-enduser-docs/spiele/blokus/_spiel_verlassen.html
        assert_eq!(
            to_string(&super::Left { room_id: "42".to_owned() }).unwrap().as_str(),
            r#"<left roomId="42"/>"#
        );
    }
}
