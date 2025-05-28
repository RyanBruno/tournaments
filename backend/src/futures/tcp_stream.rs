use std::net::TcpStream;
use std::error::Error;
use std::task::Poll;
use std::task::Context;
use std::pin::Pin;
use std::future::Future;
use std::io::ErrorKind;
use std::task::Poll::Ready;

use crate::AsyncTcpListener;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};


#[derive(Clone)]
pub struct AsyncTcpStreamFuture<'a> {
  listener: &'a AsyncTcpListener,
}
impl<'a> AsyncTcpStreamFuture<'a> {
  pub fn new(listener: &'a AsyncTcpListener) -> Self {
    AsyncTcpStreamFuture { listener }
  }
}
impl Future for AsyncTcpStreamFuture<'_> {
    type Output = Result<TcpStream, Box<dyn Error>>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
      trace!("Poll called for AsyncTcpStreamFuture");
      match self.listener.listener().accept() {
        Ok((stream, _)) => {
          trace!("Poll accepted a TcpStream");
          Ready(Ok(stream))
        },
        Err(e) if e.kind() == ErrorKind::WouldBlock => {
          trace!("Poll returned WouldBlock");
          self.listener.register(ctx.local_waker().clone());
          Poll::Pending
        },
        Err(e) => Ready(Err(Box::new(e))),
      }

    }
}