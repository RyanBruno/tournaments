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
  let mut store = KVStore::<AnEntity, AnEntity>::new(
    "data/snapshots/".into(),
    "data/events/".into(),
    10,
  )?;

  store.create("entity1".into(), AnEntity { name: "Entity One".into() })?;
  store.create("entity2".into(), AnEntity { name: "Entity Two".into() })?;
  store.update("entity1".into(), AnEntity { name: "Updated Entity One".into() })?;
  store.delete("entity2".into())?;
  store.refresh_snapshot()?;
  //let entity1 = store.read("entity1".into())?;
  //println!("Entity 1: {:?}", entity1);

  Ok(())
}