use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::task::{Context, Poll, LocalWaker};
use std::future::Future;
use std::pin::Pin;

struct Shared<T> {
    queue: VecDeque<T>,
    waker: Option<LocalWaker>,
}
pub struct Receiver<T> {
    shared: Rc<RefCell<Shared<T>>>,
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> RecvFuture<T> {
        RecvFuture {
            shared: self.shared.clone(),
        }
    }
}
pub struct RecvFuture<T> {
    shared: Rc<RefCell<Shared<T>>>,
}

impl<T> Future for RecvFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut shared = self.shared.borrow_mut();

        if let Some(value) = shared.queue.pop_front() {
            Poll::Ready(value)
        } else {
            // Save waker so `send()` can wake this future
            shared.waker = Some(cx.local_waker().clone());
            Poll::Pending
        }
    }
}
pub struct Sender<T> {
    shared: Rc<RefCell<Shared<T>>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            shared: Rc::clone(&self.shared),
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        let mut shared = self.shared.borrow_mut();
        shared.queue.push_back(value);

        // Wake any waiting receiver
        if let Some(waker) = shared.waker.take() {
            waker.wake();
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Rc::new(RefCell::new(Shared {
        queue: VecDeque::new(),
        waker: None,
    }));

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
        },
    )
}
