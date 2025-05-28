use std::fmt::Display;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct GenericError {
  msg: &'static str,
}


impl Display for GenericError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl GenericError {
  pub fn new(msg: &'static str) -> GenericError {
    GenericError {
      msg,
    }
  }
}

impl Error for GenericError {}
