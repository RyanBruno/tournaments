use std::error::Error;

use backend::database::{
  Patch, KVStore
};
use rkyv::deserialize;
use rkyv::{
    Archive, Deserialize, Serialize,
    rancor::Error as RError, 
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
  println!("Starting test...");
  let mut store = KVStore::<AnEntity, AnEntity>::new(
    "data/snapshots/".into(),
    "data/events/".into(),
    10,
  )?;

  println!("Creating and updating entities...");
  store.create("entity1".into(), AnEntity { name: "Entity One".into() })?;
  store.create("entity2".into(), AnEntity { name: "Entity Two".into() })?;
  store.update("entity1".into(), AnEntity { name: "Updated Entity One".into() })?;
  store.delete("entity2".into())?;
  println!("Refershing Snapshots.");
  store.refresh_snapshot()?;
  let entity1_archive = store.read("entity1".into())?.unwrap();
  let entity1 = deserialize::<AnEntity, RError>(entity1_archive).unwrap();

  println!("Entity 1: {:?}", entity1);
  let entity2_archive = store.read("entity2".into())?;
  assert!(entity2_archive.is_none(), "Entity 2 should not exist after deletion");

  Ok(())
}