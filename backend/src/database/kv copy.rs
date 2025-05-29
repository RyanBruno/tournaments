use std::error::Error;
use std::path::PathBuf;
use rkyv::{Serialize, Deserialize, Archive};

type EntityId = String;
pub trait Patch<T> {
    fn apply_to(self, target: &mut T);
}

struct KVStore<T>
{
  path: PathBuf,
  partition_size: usize,
  current_partition: usize,
  something: T, // Placeholder for actual data structure
}

impl<T: Archive> KVStore<T> {

  pub fn new() -> Self {
    todo!("Implement KVStore initialization logic");
  }

  pub fn create(&self, id: EntityId, obj: T) -> Result<T, Box<dyn Error>> {
    // Logic to log an event in the event log.
    todo!("Implement create logic for KVStore");
  }

  pub fn read(&self, id: EntityId) -> Result<T, Box<dyn Error>> {
    // Logic to read an object by its ID.
    todo!("Implement read logic for KVStore");
  }

  pub fn update<P>(&self, id: EntityId, patch: P) -> Result<T, Box<dyn Error>>
  where
    P: Patch<T>,
  {
    todo!("Implement patch update for KVStore");
  }

  // Additional methods for delete, update, etc. can be added here.
}