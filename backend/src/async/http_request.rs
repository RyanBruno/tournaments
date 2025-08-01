use http::{Request, Response, Method, Uri, HeaderName, HeaderValue};
use std::time::Duration;
use crate::GenericError;
use crate::AsyncTcpStream;
use crate::AsyncTcpListener;
use std::error::Error;
use crate::NetExecutor;
use crate::{Receiver, Sender, channel};


#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

struct AsyncHttpRequestInternal {
  listener: AsyncTcpListener,
  executor: NetExecutor,
  sender: Sender<(Request<()>, AsyncTcpStream)>,
}

impl AsyncHttpRequestInternal {

  async fn accept_loop_internal(&self) -> Result<(), Box<dyn Error>> {
    let stream = self.listener.accept().await?;
    let sender = self.sender.clone();

    let async_stream = AsyncTcpStream::new(
      stream,
      self.executor.clone(),
    )?;

    self.executor.clone().spawn(async move {
      let _ = AsyncHttpRequestInternal::http_handle_request(sender, async_stream).await;
    });

    Ok::<(), Box<dyn Error>>(())
  }

  pub async fn accept_loop(self) {
    loop {
      if let Err(e) = self.accept_loop_internal().await {
        error!("Error accepting connection: {e}");
        panic!("Error accepting connection: {e}");
      }
    }
  }

  async fn http_handle_request(sender: Sender<(Request<()>, AsyncTcpStream)>, mut stream: AsyncTcpStream) -> Result<(), Box<dyn Error>> {
    stream.set_timeout(Some(Duration::from_millis(250))); // 1/4 Sec

    let mut data = Vec::new();

    loop {
      match stream.read_line().await?.trim_end_matches(['\r','\n']).to_string() {
        s if s.is_empty() => {
          trace!("read_header reached the end of the header");
          break
        },
        s => {
          let len = s.len();
          trace!("Read {len} bytes from AsyncTcpStream");
          data.push(s)
        }
      };
    }
    let request = AsyncHttpRequestInternal::http_parse_request(data).ok_or(GenericError::new("Error parsing request"))?;
    let uri = request.uri();
    info!("Request recieved for uri \"{uri}\"");

    stream.set_timeout(None);

    sender.send((request, stream));

    Ok(())
  }


  fn http_parse_request(data: Vec<String>) -> Option<Request<()>> {
    let mut lines = data.into_iter();
    let request_line = lines.next()?;
    let mut parts = request_line.splitn(3, ' ');

    let method = parts.next()?.parse::<Method>().ok()?;
    let uri = parts.next()?.parse::<Uri>().ok()?;

    let mut req = Request::builder()
      .method(method)
      .uri(uri);

    for line in lines {
      if line.trim().is_empty() {
        break; // End of headers
      }
      if let Some((key, value)) = line.split_once(":") {
        let name = HeaderName::from_bytes(key.trim().as_bytes()).ok()?;
        let value = HeaderValue::from_str(value.trim()).ok()?;
        req.headers_mut()?.insert(name, value);
      }
    }

    req
      .body(())
      .ok()
  }

}

pub struct AsyncHttpRequest {
  receiver: Receiver<(Request<()>, AsyncTcpStream)>,
}

impl AsyncHttpRequest {
  pub fn new(listener: AsyncTcpListener, executor: NetExecutor) -> Self {
    let (sender, receiver) = channel();
    let internal = AsyncHttpRequestInternal {
      listener, executor: executor.clone(), sender,
    };

    executor.clone().spawn(async move {
      internal.accept_loop().await
    });

    Self {
      receiver
    }
  }

  pub async fn next_request(&self) -> Result<(Request<()>, AsyncTcpStream), Box<dyn Error>> {
    Ok(self.receiver.recv().await)
  }
  pub async fn write_response(stream: &mut AsyncTcpStream, mut response: Response<Vec<u8>>) {
    let data_length = response.body().len();

    let mut res = Vec::new();
    let code_str = response.status().canonical_reason().unwrap_or("OK").to_string();
    let code = response.status().as_u16().to_string();
    res.push(format!("HTTP/1.1 {code} {code_str}\r\n"));
    res.push(format!("Content-Length: {data_length}\r\n"));
    let headers = response.headers().into_iter()
      .filter_map(|(k, v)| match v.to_str() {
        Ok(val) => Some(format!("{k}: {val}\r\n")),
        Err(e) => {
          error!("Error parsing header value {e}");
          None
        }
      });
    res.append(&mut headers.collect());
    res.push(String::from("\r\n"));
    let mut bytes = res.join("").into_bytes();
    let data = response.body_mut();
    bytes.append(data);

    stream.write_all(&bytes).await;
  }
}