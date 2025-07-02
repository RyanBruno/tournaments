mod kv;
pub use kv::Patch;
pub use kv::KVStore;
pub use kv::EntityId;

pub mod indexed_store;
pub mod indexed_store_handle;
//pub mod store;
pub mod cqrs;