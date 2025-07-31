#![feature(local_waker)]

mod executor;
pub use crate::executor::BoxFuture;
pub use crate::executor::NetExecutor;
pub use crate::executor::NetTask;
mod r#async;
pub use crate::r#async::AsyncHttpRequest;
pub use crate::r#async::AsyncTcpListener;
pub use crate::r#async::AsyncTcpStream;
mod futures;
pub use crate::futures::AsyncReadLineFuture;
pub use crate::futures::AsyncTcpStreamFuture;
pub use crate::futures::AsyncWriteAllFuture;

mod utils;
pub use crate::utils::channel::{channel, Receiver, Sender};
pub use crate::utils::error::GenericError;
pub use crate::utils::responses::{error_route, not_found_route, serve_route};

pub mod api;
pub use crate::api::dashboard_route::dashboard_route;
pub use crate::api::dashboard_login::dashboard_login_route;
//pub use crate::api::create_event_route::create_event_route;
pub use crate::api::event_route::event_details_route;
pub use crate::api::login::login_route;
pub use crate::api::register_event_route::register_event_route;
pub use crate::api::platform_create_route::platform_create_route;
pub use crate::api::platform_update_route::platform_update_route;
pub use crate::api::platform_get_route::platform_get_route;
pub use crate::api::dashboard_profile_route::{
    dashboard_profile_get_route, dashboard_profile_patch_route,
};

pub mod database;
pub use crate::database::cqrs_store::{CQRSStore, Command};
pub use crate::database::kv_store::KVStore;

pub mod store;
pub use crate::store::dashboard::{
    dashboard_store, DashboardCommand, DashboardModel, DashboardStore,
};
pub use crate::store::dashboard::{
    registration_store, RegistrationCommand, RegistrationModel, RegistrationStore,
};
pub use crate::store::platform::{platform_store, PlatformCommand, PlatformModel, PlatformStore};

mod jwt;
pub use crate::jwt::{generate, verify, Claims};
