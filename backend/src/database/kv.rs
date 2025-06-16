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
    collections::swiss_table::ArchivedHashMap,
    string::ArchivedString,
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
  Deleted(EntityId),
}

pub struct KVStore<T> {
  snapshot_path: PathBuf,
  event_path: PathBuf,
  pending_events_len: usize,
  mmap: Mmap,
  _marker: std::marker::PhantomData<T>,
}

impl<T> KVStore<T>
where
  T: Archive + for<'a> rkyv::Serialize<rkyv::rancor::Strategy<rkyv::ser::Serializer<rkyv::util::AlignedVec, rkyv::ser::allocator::ArenaHandle<'a>, rkyv::ser::sharing::Share>, rkyv::rancor::Error>>
  + Default + Clone + for<'a> rkyv::bytecheck::CheckBytes<rkyv::rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::ArchiveValidator<'a>, rkyv::validation::shared::SharedValidator>, rkyv::rancor::Error>>
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
    let mut entity = self.read(id.clone())?.unwrap_or_default();
    patch.apply_to(&mut entity);

    let event = KVEvent::Updated(id.clone(), entity);
    self.write_event(&event)?;
    Ok(())
  }

  pub fn read(&self, id: EntityId) -> Result<Option<T>, Box<dyn Error>> {
    let map = access::<ArchivedHashMap<ArchivedString, T>, RError>(&self.mmap)?;

    if let Some(entity) = map.get(id.as_str()) {
      Ok(Some(entity.clone()))
    } else {
      Ok(None)
    }
  }

  pub fn delete(&mut self, id: EntityId) -> Result<(), Box<dyn Error>> {
    let event = KVEvent::Deleted(id.clone());
    self.write_event(&event)?;
    Ok(())
  }
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
      pending_events_len: 0,
      mmap,
      _marker: std::marker::PhantomData,
    })
  }

  fn refresh_snapshot(&mut self) -> Result<(), Box<dyn Error>> {
    // Logic to refresh the snapshot from the event log.
    todo!("Implement snapshot refresh logic");
  }

  fn write_event(&mut self, event: &KVEvent<T>) -> Result<(), Box<dyn Error>> {
    let filename = self.event_path.join(format!("event_{}.rkyv", self.pending_events_len));
    self.pending_events_len += 1;
    let archived = to_bytes::<RError>(event)?;
    let mut file = File::create(filename)?;
    file.write_all(&archived)?;

    Ok(())
  }
}