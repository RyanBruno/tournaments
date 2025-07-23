use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use super::r#match::Match;

#[derive(Archive, RkyvDeserialize, RkyvSerialize, SarSerialize, SarDeserialize, Debug, Clone, Default, PartialEq)]
pub struct Bracket {
    pub id: String,
    pub name: String,
    pub matches: Vec<Match>,
}
