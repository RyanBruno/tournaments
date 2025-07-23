use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize as SarDeserialize, Serialize as SarSerialize};

use crate::Patch;

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SarSerialize,
    SarDeserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
)]
pub struct Registration {
    pub id: String,
    pub event_id: String,
    pub email: String,
}

#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SarSerialize,
    SarDeserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
)]
pub struct RegistrationPatch {
    pub email: Option<String>,
}

impl Patch<Registration> for RegistrationPatch {
    fn apply_to(self, target: &mut Registration) {
        if let Some(email) = self.email {
            target.email = email;
        }
    }
}
