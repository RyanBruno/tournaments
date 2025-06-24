use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::PathBuf;
use std::error::Error;
use crate::{KVStore, Patch, EntityId};
use rkyv::{
    Archive, Deserialize, Serialize,
    access, deserialize,
    rancor::Error as RError, 
    rancor::Strategy,
    collections::swiss_table::ArchivedHashMap,
    string::ArchivedString,
    de::Pool,
    bytecheck::CheckBytes,
    validation::{Validator,
      shared::SharedValidator,
      archive::ArchiveValidator,
    },
};
use rkyv::api::high::HighSerializer;
use rkyv::util::AlignedVec;
use rkyv::ser::allocator::ArenaHandle;

pub struct IndexedStore<T, P, K>
where
  T: Archive,
  K: Eq + Hash + Clone,
  P: Patch<T>,
{
  pub kv: KVStore<T, P>,
  index: HashMap<K, HashSet<EntityId>>,
  extract_keys: fn(&T::Archived) -> K,
  extract_keys_t: fn(&T) -> K,
}

impl<T, P, K> IndexedStore<T, P, K>
where
  T: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  <T as Archive>::Archived: Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>,
  for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  P: Patch<T> + Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  for<'a> <P as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  <P as Archive>::Archived: Deserialize<P, Strategy<Pool, rkyv::rancor::Error>>,
  K: Eq + Hash + Clone,
{
  pub fn new(
    snapshot_path: PathBuf,
    event_path: PathBuf,
    partitions: usize,
    extract_keys: fn(&T::Archived) -> K,
    extract_keys_t: fn(&T) -> K,
  ) -> Result<Self, Box<dyn Error>> {
    let kv = KVStore::new(snapshot_path, event_path, partitions)?;
    let mut index: HashMap<K, HashSet<EntityId>> = HashMap::new();
    

    // Scan all entities and build index
    for shard in &kv.mmaps {
      let map = access::<ArchivedHashMap<ArchivedString, T::Archived>, RError>(shard)?;
      for (archived_id, archived_entity) in map.iter() {
        let id: EntityId = deserialize::<EntityId, RError>(archived_id)?;
        let key = extract_keys(&archived_entity);
        index.entry(key).or_default().insert(id.clone());
      }
    }

    Ok(Self {
      kv,
      index,
      extract_keys,
      extract_keys_t,
    })
  }

  pub fn create(&mut self, id: EntityId, entity: T) -> Result<(), Box<dyn Error>> {
    self.kv.create(id.clone(), entity.clone())?;
    let key = (self.extract_keys_t)(&entity);
    self.index.entry(key).or_default().insert(id.clone());
    Ok(())
  }

  pub fn update(&mut self, id: EntityId, patch: P) -> Result<(), Box<dyn Error>> {
    let old = self.kv.read(id.clone())?;
    let mut new_entity = old
      .map(|a| deserialize::<T, RError>(a))
      .transpose()?
      .unwrap_or_default();
    patch.clone().apply_to(&mut new_entity);

    // Remove old keys
    if let Some(old_entity) = old {
      let key = (self.extract_keys)(&old_entity);
      if let Some(set) = self.index.get_mut(&key) {
        set.remove(&id);
      }
    }

    // Apply update
    self.kv.update(id.clone(), patch)?;

    // Insert new keys
    let key = (self.extract_keys_t)(&new_entity);
    self.index.entry(key).or_default().insert(id.clone());

    Ok(())
  }

  pub fn delete(&mut self, id: EntityId) -> Result<(), Box<dyn Error>> {
    if let Some(entity) = self.kv.read(id.clone())? {
      let key = (self.extract_keys)(&entity);
      if let Some(set) = self.index.get_mut(&key) {
        set.remove(&id);
      }
    }

    self.kv.delete(id)
  }

  pub fn query(&self, key: &K) -> Vec<EntityId> {
    self.index.get(key).map(|s| s.iter().cloned().collect()).unwrap_or_default()
  }

  pub fn inner(&self) -> &KVStore<T, P> {
    &self.kv
  }

  pub fn inner_mut(&mut self) -> &mut KVStore<T, P> {
    &mut self.kv
  }
}
