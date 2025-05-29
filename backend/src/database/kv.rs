use std::error::Error;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::collections::HashMap;
use rkyv::{
    Archive, Deserialize, Serialize,
    access, to_bytes,
    rancor::Error as RError, 
};
use memmap2::Mmap;

pub type EntityId = String;

pub trait Patch<T> {
  fn apply_to(self, target: &mut T);
}

#[derive(Debug, Archive, Serialize, Deserialize)]
pub enum KVEvent<T> {
  Created(EntityId, T),
  Updated(EntityId, T),
}

pub struct KVStore<T> {
  snapshot_path: PathBuf,
  event_path: PathBuf,
  mmap: Mmap,
  _marker: std::marker::PhantomData<T>,
}

impl<T> KVStore<T>
where
  T: Archive + for<'a> rkyv::Serialize<rkyv::rancor::Strategy<rkyv::ser::Serializer<rkyv::util::AlignedVec, rkyv::ser::allocator::ArenaHandle<'a>, rkyv::ser::sharing::Share>, rkyv::rancor::Error>>,
{

  pub fn create(&mut self, id: EntityId, obj: T) -> Result<(), Box<dyn Error>> {
    let event = KVEvent::Created(id.clone(), obj);
    self.write_event(&event)?;
    Ok(())
  }

  pub fn update<P>(&mut self, id: EntityId, patch: P) -> Result<(), Box<dyn Error>>
  where
    P: Patch<T>,
  {
    let mut entity = self.read(id.clone())?;
    patch.apply_to(&mut entity);

    let event = KVEvent::Updated(id.clone(), entity);
    self.write_event(&event)?;
    Ok(())
  }

  pub fn read(&self, id: EntityId) -> Result<T, Box<dyn Error>> {
    let map = access::<HashMap<EntityId, T>, RError>(&self.mmap);

    todo!("Implement loading from snapshot or reconstruct from event log");
  }

  pub fn delete(&self, id: EntityId) -> Result<T, Box<dyn Error>> {
    todo!("Implement deletion logic, by marking as deleted in event log");
  }

  // Additional methods for delete, update, etc. can be added here.
}

impl<T> KVStore<T>
where
  T: Archive + for<'a> rkyv::Serialize<rkyv::rancor::Strategy<rkyv::ser::Serializer<rkyv::util::AlignedVec, rkyv::ser::allocator::ArenaHandle<'a>, rkyv::ser::sharing::Share>, rkyv::rancor::Error>>,
{
  pub fn new(snapshot_path: PathBuf, event_path: PathBuf, partition_size: usize) -> Result<Self, Box<dyn Error>> {
    let _ = create_dir_all(&snapshot_path);
    let _ = create_dir_all(&event_path);
    let file = File::open(&snapshot_path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    Ok(Self {
      snapshot_path,
      event_path,
      mmap,
      _marker: std::marker::PhantomData,
    })
  }

  fn refresh_snapshot(&mut self) -> Result<(), Box<dyn Error>> {
    // Logic to refresh the snapshot from the event log.
    todo!("Implement snapshot refresh logic");
  }

  fn write_event(&mut self, event: &KVEvent<T>) -> Result<(), Box<dyn Error>> {
    let filename = self.event_path.join(format!("event_{}.rkyv", 1)); // todo event id management
    let archived = to_bytes::<RError>(event)?;
    let mut file = File::create(filename)?;
    file.write_all(&archived)?;

    Ok(())
  }
}