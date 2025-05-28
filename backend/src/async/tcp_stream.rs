use std::net::TcpStream;
use std::io::BufReader;
use std::io::Write;
use std::task::LocalWaker;
use std::time::Duration;

use crate::NetExecutor;
use crate::AsyncReadLineFuture;
use crate::AsyncWriteAllFuture;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub struct AsyncTcpStream {
  reader: BufReader<TcpStream>,
  executor: NetExecutor,
}

impl Drop for AsyncTcpStream {
  fn drop(&mut self) {
    self.executor.clone().unregister(self.reader.get_ref());
  }
}

impl AsyncTcpStream {
  pub fn new(stream: TcpStream, executor: NetExecutor) -> Result<Self, Box<dyn std::error::Error>> {
    trace!("Setting TcpStream to non-blocking");
    stream.set_nonblocking(true)?;
    let this = AsyncTcpStream {
      reader: BufReader::new(stream),
      executor: executor.clone()
    };
    executor.preregister(this.reader.get_ref());
    Ok(this)
  }

  pub fn set_timeout(&self, duration: Option<Duration>) {
    self.executor.clone().set_timeout(self.reader.get_ref(), duration)
  }

  pub fn register(&self, waker: LocalWaker) {
    self.executor.clone().register(self.reader.get_ref(), waker)
  }

  pub fn reader(&mut self) -> &mut BufReader<TcpStream> {
    &mut self.reader
  }

  pub fn read_line(&mut self) -> AsyncReadLineFuture {
    AsyncReadLineFuture::new(self)
  }

  pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
    self.reader.get_ref().write(buf)
  }

  pub fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> AsyncWriteAllFuture<'a> {
    AsyncWriteAllFuture::new(self, buf)
  }
}