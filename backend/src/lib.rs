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
pub use crate::api::dashboard_route::dashboard_route;
//pub use crate::api::create_event_route::create_event_route;
pub use crate::api::event_route::event_details_route;
pub use crate::api::login::login_route;

pub mod database;
pub use crate::database::kv_store::KVStore;
pub use crate::database::cqrs_store::{CQRSStore, Command};

pub mod models;
pub use crate::models::dashboard::{DashboardView, ArchivedDashboardView, DashboardCommand, DashboardModel, DashboardStore, dashboard_store};
pub use crate::models::platform::{PlatformCommand, PlatformModel, PlatformStore, platform_store};