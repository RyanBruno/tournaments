use std::path::PathBuf;
use std::fs::OpenOptions;
use rkyv::{
    Archive, Deserialize, Serialize,
    access, to_bytes, deserialize,
    rancor::Error as RError, 
    rancor::Strategy,
    de::Pool,
    bytecheck::CheckBytes,
    validation::{Validator,
      shared::SharedValidator,
      archive::ArchiveValidator,
    },
};
use std::error::Error;
use rkyv::api::high::HighSerializer;
use rkyv::util::AlignedVec;
use rkyv::ser::allocator::ArenaHandle;
use std::io::Write;
use std::{
  fs::{File},
  io::{BufReader, Read},
};

use crate::{
  KVStore,
  Patch,
  EntityId,
};

pub struct CQRSStore<C, T, P: Patch<T>>
{
    transactions_path: PathBuf,
    kv_store: KVStore<T, P>,
    _marker_c: std::marker::PhantomData<C>,
}

pub trait Command<T, P: Patch<T>> {
  fn fold(&self, kv_store: &mut KVStore<T, P>) -> Result<(), Box<dyn Error>>;
}

impl<C, T, P> CQRSStore<C, T, P>
where
  C: Command<T, P>,
  C: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  <C as Archive>::Archived: Deserialize<C, Strategy<Pool, rkyv::rancor::Error>>,
  for<'a> <C as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  T: Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  <T as Archive>::Archived: Deserialize<T, Strategy<Pool, rkyv::rancor::Error>>,
  for<'a> <T as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  P: Patch<T> + Archive + Default + Clone
    + for<'a> Serialize<HighSerializer<AlignedVec, ArenaHandle<'a>, RError>>,
  for<'a> <P as Archive>::Archived: CheckBytes<Strategy<Validator<ArchiveValidator<'a>, SharedValidator>, rkyv::rancor::Error>>,
  <P as Archive>::Archived: Deserialize<P, Strategy<Pool, rkyv::rancor::Error>>
{
  pub fn new(transactions_path: PathBuf, kv_store: KVStore<T, P>) -> Self
  {
      CQRSStore {
          transactions_path,
          kv_store,
          _marker_c: std::marker::PhantomData,
      }
  }

  pub fn command(&mut self, command: &C) -> Result<(), Box<dyn std::error::Error>> {
    // Open the log file in append mode
    let file_path = self.transactions_path.join("transactions_log.rkyv");
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(file_path)?;

    // Archive the command
    let archived = to_bytes::<RError>(command)?;

    // Prefix with length (u32 LE)
    let len = archived.len() as u32;
    let len_bytes = len.to_le_bytes(); // Converts to [u8; 4] in little-endian
    file.write_all(&len_bytes)?;
    file.write_all(&archived)?;

    Ok(())
  }

  pub fn query(&self, id: EntityId) -> Result<Option<&<T as Archive>::Archived>, Box<dyn Error>>
  {
    self.kv_store.read(id)
  }

  pub fn fold(&mut self) -> Result<(), Box<dyn Error>> {
    let file_path = self.transactions_path.join("transactions_log.rkyv");

    let mut events: Vec<C> = Vec::new();

    // === Read Events from Log ===
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

        let archived = access::<C::Archived, RError>(&buffer[..])?;
        let event = deserialize::<C, RError>(archived)?;
        events.push(event);
      }
    }

    // === Clear Event Log ===
    File::create(&file_path)?; // truncates

    // === Apply Events ===
    events.iter().try_for_each(|event| {
      event.fold(&mut self.kv_store)
    })?;

    self.kv_store.refresh_snapshot()?;

    Ok(())
  }

}