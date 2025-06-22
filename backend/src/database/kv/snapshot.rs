use std::{
  collections::{HashMap, hash_map::DefaultHasher},
  error::Error,
  fs::{File, OpenOptions},
  hash::{Hash, Hasher},
  io::{BufReader, Read, Write},
};

use memmap2::{Mmap, MmapMut};
use rkyv::{
    Archive, Deserialize, Serialize,
    access, to_bytes, deserialize,
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
use crate::database::kv::{KVStore, KVEvent, ArchivedKVEvent, Patch}; // adjust as needed

impl<T, P> KVStore<T, P>
where
  T: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  //for<'a> &'a T: Default,
  <T as Archive>::Archived: Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>,
  for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  P: Patch<T> + Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  for<'a> <P as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  <P as Archive>::Archived: Deserialize<P, Strategy<Pool, rkyv::rancor::Error>>
{
  pub fn refresh_snapshot(&mut self) -> Result<(), Box<dyn Error>> {
    let file_path = self.event_path.join("event_log.rkyv");
    let mut shard_events: HashMap<usize, Vec<KVEvent<T, P>>> = HashMap::new();

    // === Read and Group Events ===
    {
      let mut reader = BufReader::new(File::open(&file_path)?);
      let mut buffer = Vec::new();

      loop {
        let mut len_buf = [0u8; 4];
        if reader.read_exact(&mut len_buf).is_err() {
          break;
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        buffer.resize(len, 0);
        reader.read_exact(&mut buffer)?;

        let archived = access::<ArchivedKVEvent<T, P>, RError>(&buffer[..])?;
        let event = deserialize::<KVEvent<T, P>, RError>(archived).unwrap();

        let id = match &event {
          KVEvent::Created(id, _) |
          KVEvent::Updated(id, _) |
          KVEvent::Deleted(id) => id,
        };

        let shard_index = self.compute_shard_index(&id);
        shard_events.entry(shard_index).or_default().push(event);
      }
    }

    // === Clear Event Log ===
    File::create(&file_path)?; // truncates

    // === Process Each Shard ===
    for (shard_index, events) in shard_events {
      let path = self.snapshot_path.join(format!("partition_{shard_index}.rkyv"));

      // Drop mmap to release the file lock before writing
      let dummy_map = MmapMut::map_anon(1)?.make_read_only()?;
      let old_map = std::mem::replace(&mut self.mmaps[shard_index], dummy_map); // replace with dummy
      drop(old_map); // drop the old mmap to release the file lock

      // Load old snapshot from file
      let file = File::open(&path)?;
      let mmap = unsafe { Mmap::map(&file)? };
      let archived = access::<ArchivedHashMap<ArchivedString, T::Archived>, RError>(&mmap)?;
      let mut map = deserialize::<HashMap<String, T>, RError>(archived)?;

      // Apply events
      for event in events {
        match event {
          KVEvent::Created(id, val) => {
            map.insert(id, val);
          }
          KVEvent::Updated(id, val) => {
            let mut entity = map.remove(&id).unwrap_or_default();
            val.apply_to(&mut entity);
            map.insert(id, entity);
          }
          KVEvent::Deleted(id) => {
            map.remove(&id);
          }
        }
      }

      // Write new snapshot
      let bytes = to_bytes::<RError>(&map)?;
      let mut file = File::create(&path)?;
      file.write_all(&bytes)?;
      file.flush()?; // ensure everything is written

      // Refresh mmap
      let file = OpenOptions::new().read(true).open(&path)?;
      let mmap = unsafe { Mmap::map(&file)? };
      self.mmaps[shard_index] = mmap;
    }


    Ok(())
  }

  fn compute_shard_index(&self, id: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    (hasher.finish() % self.mmaps.len() as u64) as usize
  }
}
