use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::Patch;

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct Match {
    pub id: String,
    pub participant_a: String,
    pub participant_b: String,
    pub score_a: u32,
    pub score_b: u32,
    pub completed: bool,
}

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct ScoreUpdate {
    pub match_id: String,
    pub score_a: u32,
    pub score_b: u32,
}

impl Patch<Match> for ScoreUpdate {
    fn apply_to(self, target: &mut Match) {
        if target.id == self.match_id {
            target.score_a = self.score_a;
            target.score_b = self.score_b;
            target.completed = true;
        }
    }
}
