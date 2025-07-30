use std::error::Error;
use std::future::Future;
use std::io::{BufRead, ErrorKind};
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::task::Poll::Ready;

use crate::AsyncTcpStream;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct AsyncReadLineFuture<'a> {
    stream: &'a mut AsyncTcpStream,
    buf: String,
}
impl<'a> AsyncReadLineFuture<'a> {
    pub fn new(stream: &'a mut AsyncTcpStream) -> Self {
        AsyncReadLineFuture {
            stream,
            buf: String::new(),
        }
    }
}
impl Future for AsyncReadLineFuture<'_> {
    type Output = Result<String, Box<dyn Error>>;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        trace!("Poll called for AsyncReadLineFuture");

        loop {
            let mut local = std::mem::take(&mut self.buf);
            let mut consume = 0usize;
            let mut complete = false;

            {
                let reader = self.stream.reader();
                match reader.fill_buf() {
                    Ok(buf) if buf.is_empty() => {
                        complete = true;
                    }
                    Ok(buf) => {
                        if let Some(pos) = buf.iter().position(|b| *b == b'\n') {
                            let slice = &buf[..=pos];
                            local.push_str(
                                std::str::from_utf8(slice)
                                    .map_err(|e| Box::new(e) as Box<dyn Error>)?,
                            );
                            consume = pos + 1;
                            complete = true;
                        } else {
                            local.push_str(
                                std::str::from_utf8(buf)
                                    .map_err(|e| Box::new(e) as Box<dyn Error>)?,
                            );
                            consume = buf.len();
                        }
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        self.buf = local;
                        self.stream.register(ctx.local_waker().clone());
                        return Poll::Pending;
                    }
                    Err(e) => return Ready(Err(Box::new(e))),
                }
            }

            if consume > 0 {
                self.stream.reader().consume(consume);
            }

            if complete {
                trace!("Poll read a line {local}");
                return Ready(Ok(local));
            } else {
                self.buf = local;
            }
        }
    }
}
