use std::error::Error;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::io::BufReader;
use std::io::Read;
use rkyv::{
    Archive, Deserialize, Serialize,
    access, to_bytes, deserialize,
    rancor::Error as RError, 
    rancor::Strategy,
    collections::swiss_table::ArchivedHashMap,
    string::ArchivedString,
    bytecheck::CheckBytes,
    de::Pool,
    validation::{Validator,
      shared::SharedValidator,
      archive::ArchiveValidator,
    },
};
use models::{
  Patch,
  EntityId,
};
use rkyv::api::high::HighSerializer;
use rkyv::util::AlignedVec;
use rkyv::ser::allocator::ArenaHandle;
use memmap2::{Mmap, MmapMut};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs::OpenOptions;
use std::collections::HashMap;

pub struct KVStore<T, P: Patch<T>> {
  snapshot_path: PathBuf,
  event_path: PathBuf,
  pub mmaps: Vec<Mmap>,
  _marker_t: std::marker::PhantomData<T>,
  _marker_p: std::marker::PhantomData<P>,
}

#[derive(Debug, Archive, Serialize, Deserialize)]
enum KVEvent<T, P: Patch<T>> {
  Created(EntityId, T),
  Updated(EntityId, P),
  Deleted(EntityId),
}

impl<T, P> KVStore<T, P>
where
  T: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  <T as Archive>::Archived: Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>,
  for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  P: Patch<T> + Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  for<'a> <P as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  <P as Archive>::Archived: Deserialize<P, Strategy<Pool, rkyv::rancor::Error>>
{

  pub fn create(&mut self, id: EntityId, obj: T) -> Result<(), Box<dyn Error>> {
    let event = KVEvent::Created(id.clone(), obj);
    self.write_event(&event)?;
    Ok(())
  }

  pub fn update(&mut self, id: EntityId, patch: P) -> Result<(), Box<dyn Error>>
  {
    let event = KVEvent::Updated(id.clone(), patch);
    self.write_event(&event)?;
    Ok(())
  }

  pub fn delete(&mut self, id: EntityId) -> Result<(), Box<dyn Error>> {
    let event = KVEvent::Deleted(id.clone());
    self.write_event(&event)?;
    Ok(())
  }

  pub fn read(&self, id: EntityId) -> Result<Option<&<T as Archive>::Archived>, Box<dyn Error>>
  {
    let shard_index = {
      let mut hasher = DefaultHasher::new();
      id.hash(&mut hasher);
      (hasher.finish() % self.mmaps.len() as u64) as usize
    };

    let mmap = &self.mmaps[shard_index];
    let map = access::<ArchivedHashMap<ArchivedString, T::Archived>, RError>(mmap)?;

    if let Some(entity) = map.get(id.as_str()) {
      Ok(Some(entity))
    } else {
      Ok(None)
    }
  }

  pub fn new(snapshot_path: PathBuf, event_path: PathBuf, partitions: usize) -> Result<Self, Box<dyn Error>> {
    // Ensure the snapshot and event directories exist
    let _ = create_dir_all(&snapshot_path);
    let _ = create_dir_all(&event_path);

    // Load the snapshot file
    let mut mmaps = Vec::with_capacity(partitions);
    for i in 0..partitions {
        let path = snapshot_path.join(format!("partition_{i}.rkyv"));
        if !path.exists() {
            let map: HashMap<String, T> = HashMap::new();
            let archived = to_bytes::<RError>(&map)?;
            let mut file = File::create(&path)?;
            file.write_all(&archived)?;
        }
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        mmaps.push(mmap);
    }

    Ok(Self {
      snapshot_path,
      event_path,
      mmaps,
      _marker_t: std::marker::PhantomData,
      _marker_p: std::marker::PhantomData,
    })
  }

  pub fn _refresh_snapshot(&mut self) -> Result<(), Box<dyn Error>>
    where for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>
  {
    let file_path = self.event_path.join("event_log.rkyv");
    {
      let mut reader = BufReader::new(File::open(&file_path)?);
      let mut buffer = Vec::new();

      loop {
        let mut len_buf = [0u8; 4];
        if reader.read_exact(&mut len_buf).is_err() {
          break; // Reached EOF or corrupt data
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        buffer.resize(len, 0);
        reader.read_exact(&mut buffer)?;

        //let archived = unsafe { archived_root::<KVEvent<T>>(&buffer[..]) };
        let event = access::<ArchivedKVEvent<T, P>, RError>(&buffer[..])?;

        match event {
          ArchivedKVEvent::Created(id, _obj) => {
            // Handle creation
            println!("Create {id}");
          },
          ArchivedKVEvent::Updated(id, _obj) => {
            println!("Update {id}");
          },
          ArchivedKVEvent::Deleted(id) => {
            println!("Delete {id}");
          },
        };
      };
    }
    OpenOptions::new()
      .write(true)
      .truncate(true) // this clears the contents
      .open(file_path)?; 

    Ok(())
  }

  fn write_event(&mut self, event: &KVEvent<T, P>) -> Result<(), Box<dyn Error>> {
    // Open the log file in append mode
    let file_path = self.event_path.join("event_log.rkyv");
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(file_path)?;

    // Archive the event
    let archived = to_bytes::<RError>(event)?;

    // Prefix with length (u32 LE)
    let len = archived.len() as u32;
    let len_bytes = len.to_le_bytes(); // Converts to [u8; 4] in little-endian
    file.write_all(&len_bytes)?;
    file.write_all(&archived)?;

    Ok(())
  }
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

        let shard_index = self.compute_shard_index(id);
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