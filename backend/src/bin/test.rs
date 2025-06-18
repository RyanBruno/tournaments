use std::error::Error;

use backend::database::{
  Patch, KVStore
};
use rkyv::{
    Archive, Deserialize, Serialize,
};

#[derive(Archive, Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
struct AnEntity {
  name: String,
}

impl Patch<AnEntity> for AnEntity {
  fn apply_to(self, target: &mut AnEntity) {
    target.name = self.name;
  }
}

pub fn main() -> Result<(), Box<dyn Error>> {
  let _store = KVStore::<AnEntity, AnEntity>::new(
    "data/snapshots/".into(),
    "data/events/".into(),
    10,
  );
  Ok(())
}