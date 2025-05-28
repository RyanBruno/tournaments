use std::pin::Pin;
use std::future::Future;

mod net_executor;
mod net_task;

pub use net_executor::NetExecutor;
pub use net_task::NetTask;
pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;