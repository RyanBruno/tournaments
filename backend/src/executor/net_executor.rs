use std::task::ContextBuilder;
use std::time::Instant;
use std::time::Duration;
use std::task::LocalWaker;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::os::fd::AsRawFd;
use std::task::Waker;
use polling::{Event, Events, Poller};
use polling::AsSource;
use polling::AsRawSource;
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

use crate::NetTask;

//#[derive(Default)]
struct NetExecutorInner {
  poller: Poller,
  map: HashMap<usize, LocalWaker>,
  queue: Vec<NetTask>,
  timeouts: HashMap<usize, (Instant, Duration)>,
}

#[derive(Clone)]
pub struct NetExecutor(Rc<RefCell<NetExecutorInner>>);

impl Default for NetExecutor {
  fn default() -> Self {
    Self::new()
  }
}

impl NetExecutor {
  pub fn new() -> Self {
    NetExecutor(Rc::new(RefCell::new(NetExecutorInner {
      poller: Poller::new().unwrap(),
      map: HashMap::new(),
      queue: Vec::new(),
      timeouts: HashMap::new(),
    })))
  }

  pub fn set_timeout(self, fd: impl AsSource, time: Option<Duration>) {
    match time {
      Some(d) => {
        self.0.borrow_mut().timeouts.insert(fd.source().as_raw_fd().try_into().unwrap(), (Instant::now(), d));
      },
      None => {
        self.0.borrow_mut().timeouts.remove(&fd.source().as_raw_fd().try_into().unwrap());
      },
    }

  }

  pub fn enqueue(self, task: NetTask) {
    trace!("Queueing Task");
    self.0.borrow_mut().queue.push(task);
    trace!("Queued Task");
  }

  pub fn preregister(self, raw: impl AsRawSource) {
    let fd: usize = raw.raw().try_into().unwrap();

    unsafe {
      self.0.borrow().poller.add(raw, Event::none(fd)).unwrap();
      trace!("Added fd #{fd} to Poller");
    }
  }

  pub fn unregister(self, raw: impl AsSource) {
    let fd: usize = raw.source().as_raw_fd().try_into().unwrap();
    trace!("Unregistering fd #{fd} from poller");
    self.0.borrow().poller.delete(&raw).unwrap();
    self.set_timeout(raw, None);
  }

  pub fn register(self, raw: impl AsSource, waker: LocalWaker) {
    let fd: usize = raw.source().as_raw_fd().try_into().unwrap();
    trace!("Registering fd #{fd} with NetExecutor");
    self.0.borrow_mut().map.insert(fd, waker);
    trace!("Added fd #{fd} to NetExecutor's Map");

    self.0.borrow().poller.modify(raw, Event::readable(fd)).unwrap();
    trace!("Added fd #{fd} to Poller");
  }

  pub fn spawn(self, future: impl Future<Output = ()> + 'static) {
    let future = Box::pin(future);
    trace!("Spawning task with NetExecutor");
    let task = NetTask::new(future, self.clone());
    self.enqueue(task);
    trace!("Added task to NetExecutor's queue");
  }

  pub fn run(self) {
    loop {
      loop {
        let mut queue = std::mem::take(&mut self.0.borrow_mut().queue);
        let queue_size = queue.len();
        if queue_size == 0 { break }

        trace!("Starting NetExecutor loop with {queue_size} tasks");
        for task in queue.drain(0..) {
          let task: Rc<NetTask> = Rc::new(task);
          let _ = unsafe { task.poll(&mut ContextBuilder::from_waker(Waker::noop())
            .local_waker(&task.clone().into())
            .build()) } ;
        }
      }

      trace!("Polling");
      let events = {
        let mut events = Events::new();
        self.0.borrow().poller.wait(&mut events, Some(Duration::new(100000, 0))).unwrap();
        events
      };
      let events_len = events.len();
      trace!("Recieved {events_len} events from Poller");
      for event in events.iter() {
        let fd = event.key;
        trace!("Recieved an event from Poller {event:?}");
        let waker = self.0.borrow_mut().map.remove(&fd).unwrap();
        waker.wake();
        trace!("Called wake on fd #{fd}");
      }

      trace!("Reviewing timeouts");
      let timeouts = self.0.borrow().timeouts.iter()
        .filter_map(|(k, (i, d))|
          match i.elapsed() > *d {
            true => Some(*k),
            false => None,
        })
        .collect::<Vec<usize>>();

      let timeouts_len = timeouts.len();
      trace!("Timing out {timeouts_len} fds");
      let _ = timeouts.iter()
        .filter_map(|fd| self.0.borrow_mut().map.remove(fd))
        .collect::<Vec<LocalWaker>>();
    }
  }
}