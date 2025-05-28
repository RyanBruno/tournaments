use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::future::Future;
use std::io::ErrorKind;
use std::error::Error;
use std::task::Poll::Ready;
use std::io::BufRead;

use crate::AsyncTcpStream;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

pub struct AsyncReadLineFuture<'a> {
  stream: &'a mut AsyncTcpStream,
}
impl<'a> AsyncReadLineFuture<'a> {
  pub fn new(stream: &'a mut AsyncTcpStream) -> Self {
    AsyncReadLineFuture { stream }
  }
}
impl Future for AsyncReadLineFuture<'_> {
    type Output = Result<String, Box<dyn Error>>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
      trace!("Poll called for AsyncReadLineFuture");
      let reader = self.stream.reader();
      let mut buffer = String::new();

      // TODO this blocks :(
      match reader.read_line(&mut buffer) {
        Ok(_) => {
          trace!("Poll read a line {buffer}");
          Ready(Ok(buffer))
        },
        Err(e) if e.kind() == ErrorKind::WouldBlock => {
          trace!("Poll returned WouldBlock");
          self.stream.register(ctx.local_waker().clone());
          Poll::Pending
        },
        Err(e) => Ready(Err(Box::new(e))),
      }
    }
}