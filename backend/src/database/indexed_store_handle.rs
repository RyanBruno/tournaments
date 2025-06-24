use std::rc::Rc;
use std::cell::RefCell;
use crate::{
  Patch,
  database::indexed_store::IndexedStore,
  EntityId,
  KVStore,
};
use std::error::Error;
use std::hash::Hash;
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
use std::path::PathBuf;

#[derive(Clone)]
pub struct IndexedStoreHandle<T, P, K>
where
  T: Archive,
  K: Eq + Hash + Clone,
  P: Patch<T>,
{
  inner: Rc<RefCell<IndexedStore<T, P, K>>>,
}

impl<T, P, K> IndexedStoreHandle<T, P, K>
where
  T: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  <T as Archive>::Archived: Deserialize<T, Strategy<Pool, RError>>,
  for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, RError>>,
  P: Patch<T> + Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  for<'a> <P as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, RError>>,
  <P as Archive>::Archived: Deserialize<P, Strategy<Pool, RError>>,
  K: Eq + Hash + Clone,
{
  pub fn new(
    snapshot_path: PathBuf,
    event_path: PathBuf,
    partitions: usize,
    extract_keys: fn(&T::Archived) -> K,
    extract_keys_t: fn(&T) -> K,
  ) -> Result<Self, Box<dyn Error>> {
    let store = IndexedStore::new(snapshot_path, event_path, partitions, extract_keys, extract_keys_t)?;
    Ok(Self {
      inner: Rc::new(RefCell::new(store)),
    })
  }

  pub fn create(&self, id: EntityId, entity: T) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().create(id, entity)
  }

  pub fn update(&self, id: EntityId, patch: P) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().update(id, patch)
  }

  pub fn delete(&self, id: EntityId) -> Result<(), Box<dyn Error>> {
    self.inner.borrow_mut().delete(id)
  }

  pub fn query(&self, key: &K) -> Vec<EntityId> {
    self.inner.borrow().query(key)
  }

  pub fn kv(&self) -> std::cell::Ref<KVStore<T, P>> {
    std::cell::Ref::map(self.inner.borrow(), |s| &s.kv)
  }

  pub fn kv_mut(&self) -> std::cell::RefMut<KVStore<T, P>> {
    std::cell::RefMut::map(self.inner.borrow_mut(), |s| &mut s.kv)
  }

  pub fn raw(&self) -> Rc<RefCell<IndexedStore<T, P, K>>> {
    Rc::clone(&self.inner)
  }
}
