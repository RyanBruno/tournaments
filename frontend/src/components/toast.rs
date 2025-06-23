use dioxus::prelude::*;
use crate::{ToastContext, ToastKind};

#[component]
pub fn Toast() -> Element {
  let toast = use_context::<Signal<ToastContext>>();

  if let Some(toast) = toast().toast {
    let color = match toast.kind {
      ToastKind::Error => "#f87171",
      ToastKind::Success => "#4ade80",
      ToastKind::Info => "#60a5fa",
    };

    rsx!(
      div {
        style: format!(
          "
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            background-color: {color};
            color: white;
            padding: 0.75rem 1.25rem;
            border-radius: 0.5rem;
            font-weight: 300;
            font-size: 0.875rem;
            box-shadow: 0 4px 12px rgba(0,0,0,0.1);
            z-index: 9999;
            animation: fadein 0.3s ease;
          "
        ),
        "{toast.message}"
      }
    )
  } else {
    rsx!()
  }
}
