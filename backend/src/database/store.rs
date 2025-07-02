use std::path::PathBuf;
use std::error::Error;
use crate::{KVStore, Patch};
use rkyv::{
    Archive, Deserialize, Serialize,
    rancor::Error as RError, 
    rancor::Strategy,
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



pub struct DataStore<T, P>
where
  T: Archive,
  P: Patch<T>,
{
  store: KVStore<T, P>,
}

impl<T, P> DataStore<T, P>
where
  T: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  <T as Archive>::Archived: Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>,
  for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  P: Patch<T> + Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  for<'a> <P as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  <P as Archive>::Archived: Deserialize<P, Strategy<Pool, rkyv::rancor::Error>>,
{
  pub fn new(
    snapshot_path: PathBuf,
    event_path: PathBuf,
    partitions: usize,
  ) -> Result<Self, Box<dyn Error>> {
    let store = KVStore::<T, P>::new(snapshot_path, event_path, partitions)?;

    Ok(DataStore {
      store,
    })
  }
}

pub trait StoreModel<K, V> {
  /// Queue a mutation event into the model’s internal log
  fn write_event(&mut self, event: V);

  /// Read one or more keys from the internal store
  fn read(&self, key: &K) -> Option<V>;

  /// Apply queued mutations to the underlying IndexedKvStore
  fn sync(&mut self);

  /// Get all entity IDs associated with a key
  fn query_ids(&self, key: &K) -> Vec<String>;

  /// Get all denormalized readable objects associated with a key
  fn query(&self, key: &K) -> Vec<V>;

  /// Reset the local mutation buffer (useful in tests or dry runs)
  fn clear_pending(&mut self);
}
