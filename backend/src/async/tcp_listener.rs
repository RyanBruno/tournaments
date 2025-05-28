use std::os::fd::AsRawFd;
use std::net::TcpListener;
use std::task::LocalWaker;

use crate::NetExecutor;
use crate::AsyncTcpStreamFuture;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub struct AsyncTcpListener {
  listener: TcpListener,
  executor: NetExecutor,
}
impl Drop for AsyncTcpListener {
  fn drop(&mut self) {
    self.executor.clone().unregister(&self.listener);
  }
}
impl AsyncTcpListener {
  pub fn new(port: u16, executor: NetExecutor) -> Result<Self, Box<dyn std::error::Error>> {
    trace!("Opening TcpListener on port {port}");
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    trace!("Setting TcpListener to non-blocking");
    listener.set_nonblocking(true)?;
    let fd: usize = listener.as_raw_fd().try_into()?;
    trace!("Successfully opened TcpListener on port {port} (#{fd})");
    let this = AsyncTcpListener {
      listener,
      executor: executor.clone()
    };
    executor.preregister(&this.listener);
    Ok(this)
  }

  pub fn register(&self, waker: LocalWaker) {
    self.executor.clone().register(&self.listener, waker)
  }

  pub fn listener(&self) -> &TcpListener {
    &self.listener
  }

  pub fn accept(&self) -> AsyncTcpStreamFuture {
    AsyncTcpStreamFuture::new(self)
  }
}