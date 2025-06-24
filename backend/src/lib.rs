#![feature(local_waker)]

mod executor;
pub use crate::executor::NetExecutor;
pub use crate::executor::NetTask;
pub use crate::executor::BoxFuture;
mod r#async;
pub use crate::r#async::AsyncTcpListener;
pub use crate::r#async::AsyncTcpStream;
pub use crate::r#async::AsyncHttpRequest;
mod futures;
pub use crate::futures::AsyncTcpStreamFuture;
pub use crate::futures::AsyncReadLineFuture;
pub use crate::futures::AsyncWriteAllFuture;

mod utils;
pub use crate::utils::channel::{Receiver, Sender, channel};
pub use crate::utils::error::GenericError;
pub use crate::utils::responses::{
    serve_route,
    not_found_route,
    error_route,
};

pub mod api;
pub use crate::api::dashboard::dashboard_route;
pub use crate::api::create_event_route::create_event_route;

pub mod database;
pub use crate::database::KVStore;
pub use crate::database::Patch;
pub use crate::database::EntityId;
pub use crate::database::indexed_store_handle::IndexedStoreHandle;

pub mod models;
pub use crate::models::events::{Event, EventPatch, event_store};