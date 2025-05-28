use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::future::Future;
use std::io::ErrorKind;
use std::task::Poll::Ready;

use crate::AsyncTcpStream;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub struct AsyncWriteAllFuture<'a> {
  stream: &'a mut AsyncTcpStream,
  buf: &'a [u8],
  cursor: usize,
}
impl<'a> AsyncWriteAllFuture<'a> {
  pub fn new(stream: &'a mut AsyncTcpStream, buf: &'a [u8]) -> Self {
    AsyncWriteAllFuture { stream, buf, cursor: 0 }
  }
}
impl Future for AsyncWriteAllFuture<'_> {
  type Output = Option<()>;

  fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
    trace!("Poll called for AsyncWriteAllFuture");

    match self.stream.write(&self.buf[self.cursor..]).map_err(|e| e.kind()) {
      Ok(s) => {
        trace!("Poll wrote {s} bytes");
        self.cursor += s;
        match self.cursor == self.buf.len() {
          true => {
            trace!("AsyncWriteAllFuture is done writing");
            Ready(Some(()))
          },
          false => {
            trace!("AsyncWriteAllFuture has more bytes to write");
            self.stream.register(ctx.local_waker().clone());
            Poll::Pending
          },
        }
      },
      Err(ErrorKind::WouldBlock) => {
        trace!("Poll returned WouldBlock");
        self.stream.register(ctx.local_waker().clone());
        Poll::Pending
      },
      Err(_) => {
        // Does not log or panic on error
        // I assume the connection closed
        Ready(None)
      },
    }
  }
}