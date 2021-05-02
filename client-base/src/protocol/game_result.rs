use serde::{Serialize, Deserialize};
use super::{PlayerScore, ScoreDefinition};

/// The final result of a game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameResult<P> {
    pub definition: ScoreDefinition,
    #[serde(rename = "score")]
    pub scores: Vec<PlayerScore>,
    #[serde(rename = "winner")]
    pub winners: Vec<P>
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use quick_xml::de::from_str;

    use crate::{plugin::{MockPlayer, MockPlayerColor}, protocol::{PlayerScore, ScoreAggregation, ScoreCause, ScoreDefinition, ScoreFragment}};

    use super::GameResult;

    #[test]
    fn test_deserialization() {
        // See https://cau-kiel-tech-inf.github.io/socha-enduser-docs/spiele/blokus/spielende.html
        let raw = indoc! {r#"
            <data class="result">
                <definition>
                    <fragment name="Winner">
                        <aggregation>SUM</aggregation>
                        <relevantForRanking>true</relevantForRanking>
                    </fragment>
                    <fragment name="Average points">
                        <aggregation>AVERAGE</aggregation>
                        <relevantForRanking>true</relevantForRanking>
                    </fragment>
                </definition>
                <score cause="REGULAR" reason="Player won the game">
                    <part>2</part>
                    <part>82</part>
                </score>
                <score cause="HARD_TIMEOUT" reason="Player did not respond in time">
                    <part>0</part>
                    <part>42</part>
                </score>
                <winner displayName="Alex">
                    <color class="team">RED</color>
                </winner>
            </data>
        "#};
        let result: GameResult<MockPlayer> = from_str(raw).unwrap();
        assert_eq!(
            result,
            GameResult {
                definition: ScoreDefinition {
                    fragments: vec![
                        ScoreFragment {
                            name: "Winner".to_owned(),
                            aggregation: ScoreAggregation::Sum,
                            relevant_for_ranking: true
                        },
                        ScoreFragment {
                            name: "Average points".to_owned(),
                            aggregation: ScoreAggregation::Average,
                            relevant_for_ranking: true
                        }
                    ]
                },
                scores: vec![
                    PlayerScore {
                        cause: ScoreCause::Regular,
                        reason: "Player won the game".to_owned()
                    },
                    PlayerScore {
                        cause: ScoreCause::HardTimeout,
                        reason: "Player did not respond in time".to_owned()
                    }
                ],
                winners: vec![
                    MockPlayer {
                        color: MockPlayerColor::Red,
                        display_name: "Alex".to_owned()
                    }
                ]
            }
        )
    }
}
