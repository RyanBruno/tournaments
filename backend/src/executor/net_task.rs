use std::task::LocalWake;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::task::Context;
use std::task::Poll;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

use crate::NetExecutor;
use crate::BoxFuture;

pub struct NetTask {
  future: UnsafeCell<BoxFuture>,
  executor: NetExecutor,
}

impl NetTask {
  pub fn new(future: BoxFuture, executor: NetExecutor) -> Self {
    Self {
      future: UnsafeCell::new(future),
      executor
    }
  }

  pub unsafe fn poll(&self, ctx: &mut Context<'_>) -> Poll<()> {
    (*self.future.get()).as_mut().poll(ctx)
  }
}
impl Drop for NetTask {
  fn drop(&mut self) {
    trace!("Dropped Task")
  }
}
impl LocalWake for NetTask {
  fn wake(self: Rc<Self>) {
    self.executor.clone().enqueue(Rc::into_inner(self).unwrap());
  }
}