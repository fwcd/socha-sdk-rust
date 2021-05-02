use std::{fmt, str::FromStr};
use serde::{Serialize, Deserialize};
use crate::plugin::SCPlugin;
use super::Event;

/// A message in a room together with some data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "room", rename_all = "camelCase")]
pub struct Room<P> where P: SCPlugin {
    pub room_id: String,
    #[serde(rename = "data", bound(
        serialize = "P::Player: Serialize, P::Move: Serialize, P::PlayerColor: fmt::Display, P::GameState: Serialize",
        deserialize = "P::Player: Deserialize<'de>, P::Move: Deserialize<'de>, P::PlayerColor: FromStr, P::GameState: Deserialize<'de>"
    ))]
    pub event: Event<P>
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use quick_xml::de::from_str;
    use crate::{plugin::{MockGameState, MockPlayerColor, MockPlugin}, protocol::Event};

    use super::Room;

    // TODO: Currently throws "invalid type: string \"18\", expected u32",
    //       presumably for similar reasons as:
    //         - https://github.com/tafia/quick-xml/issues/226
    //         - https://github.com/tafia/quick-xml/issues/190
    //         - https://github.com/serde-rs/serde/issues/1183
    #[test]
    #[ignore]
    fn test_deserialization() {
        // See https://cau-kiel-tech-inf.github.io/socha-enduser-docs/spiele/blokus/spielstatus.html#memento
        let raw = indoc! {r#"
            <room roomId="123">
                <data class="memento">
                    <state turn="18">
                        <playerColor>RED</playerColor>
                    </state>
                </data>
            </room>
        "#};
        let room: Room<MockPlugin> = from_str(raw).unwrap();
        assert_eq!(
            room,
            Room {
                room_id: "123".to_owned(),
                event: Event::Memento {
                    state: MockGameState {
                        player_color: MockPlayerColor::Red,
                        turn: 18
                    }
                }
            }
        );
    }
}
