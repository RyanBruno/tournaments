
#[derive(Clone, PartialEq)]
pub struct Toast {
  pub message: String,
  pub kind: ToastKind,
}

#[derive(Clone, PartialEq)]
pub enum ToastKind {
  Error,
  Success,
  Info,
}

#[derive(Clone)]
pub struct ToastContext {
  pub toast: Option<Toast>,
}
