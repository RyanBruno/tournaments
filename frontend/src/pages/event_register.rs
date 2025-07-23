use crate::{use_event, use_register_event, ClientContext, ToastContext};
use dioxus::prelude::*;
const BUCKET_GOLF_SVG: Asset = asset!("/assets/bucket-golf.png");

#[component]
pub fn RegisterEvent(id: String) -> Element {
    let data = use_event(
        id,
        use_context::<Signal<ToastContext>>(),
        use_context::<Signal<ClientContext>>(),
    );

    match data.read_unchecked().as_ref() {
        Some(Some(event)) => rsx!(RegisterEventInner {
            id: event.id.clone(),
            name: event.name.clone(),
            location: event.location.clone(),
            date: event.date.clone(),
            image: event.image.clone(),
            banner: event.banner.clone(),
            upsell: event.upsell.clone()
        }),
        _ => rsx!(),
    }
}

#[component]
pub fn RegisterEventInner(
    id: String,
    name: String,
    location: String,
    date: String,
    image: String,
    banner: Option<String>,
    upsell: Option<String>,
) -> Element {
    let mut email = use_signal(|| String::new());
    let mut trigger = use_signal(|| None);
    let _reg = use_register_event(
        trigger,
        use_context::<Signal<ToastContext>>(),
        use_context::<Signal<ClientContext>>(),
    );

    rsx!(
      div {
        style: "
        min-height: 100vh;
        background-color: #f9fafb;
        font-family: sans-serif;
        padding: 3rem 1rem;
        display: flex;
        justify-content: center;
      ",
        div {
          style: "
          max-width: 48rem;
          width: 100%;
          background: white;
          padding: 2rem;
          border-radius: 1rem;
          box-shadow: 0 4px 12px rgba(0,0,0,0.05);
        ",

          // Banner
          if let Some(b) = banner.clone() {
            div {
              style: "background-color: #fde68a; color: #78350f; font-weight: bold; text-align: center; padding: 0.5rem; margin-bottom: 1rem;",
              "{b}"
            }
          }

          // Event header
          h1 {
            style: "font-size: 2rem; font-weight: bold; color: #111827; margin-bottom: 0.5rem;",
            "Register for {name}"
          }

          p {
            style: "color: #6b7280; margin-bottom: 1.5rem;",
            "{date} · {location}"
          }

          img {
            src: BUCKET_GOLF_SVG,
            alt: "Event image",
            style: "width: 100%; max-height: 200px; object-fit: cover; border-radius: 0.5rem; margin-bottom: 2rem;"
          }

          if let Some(u) = upsell.clone() {
            p {
              style: "color: #dc2626; font-weight: 500; font-size: 0.95rem; margin-bottom: 1.5rem;",
              "{u}"
            }
          }

          // Pricing breakdown
          div {
            style: "margin-bottom: 2rem;",
            h2 { style: "font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem;", "Order Summary" }
            ul {
              style: "font-size: 0.95rem; color: #374151; margin-bottom: 1rem;",
              li { "🏷️ Registration Fee: $20.00" }
              li { "⚙️ Service Fee: $2.00" }
              li { "📊 Taxes: $1.80" }
              li { "💖 Optional Donation: $0.00" }
            }
            p { style: "font-weight: bold; font-size: 1.125rem; margin-top: 1rem;", "Total: $23.80" }
          }

          // Auth options
          div {
            style: "margin-bottom: 2rem;",
            h2 { style: "font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem;", "Your Account" }

            input {
              r#type: "email",
              placeholder: "Email address",
              oninput: move |e| {
                email.set(e.value());
              },
              value: email,
              style: base_input()
            }

            input {
              r#type: "password",
              placeholder: "Create a password or enter existing",
              style: base_input()
            }

            a {
              href: "#",
              style: "font-size: 0.875rem; color: #4f46e5; text-decoration: underline;",
              "Already have an account? Log in"
            }
          }

          // Billing address
          div {
            style: "margin-bottom: 2rem;",
            h2 { style: "font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem;", "Billing Address" }

            input { placeholder: "Full Name", style: base_input() }
            input { placeholder: "Street Address", style: base_input() }
            input { placeholder: "City", style: base_input() }
            input { placeholder: "State", style: base_input() }
            input { placeholder: "Zip Code", style: base_input() }
          }

          // Payment section placeholder
          div {
            style: "margin-bottom: 2rem;",
            h2 { style: "font-size: 1.125rem; font-weight: 600; margin-bottom: 1rem;", "Payment" }
            div {
              style: "
              padding: 1.5rem;
              border: 2px dashed #d1d5db;
              border-radius: 0.5rem;
              text-align: center;
              color: #9ca3af;
              font-size: 0.95rem;
            ",
              "💳 Payment integration coming soon..."
            }
          }

          button {
            style: "
            width: 100%;
            background-color: #4f46e5;
            color: white;
            font-weight: 600;
            font-size: 1rem;
            padding: 0.75rem;
            border: none;
            border-radius: 0.5rem;
            cursor: pointer;
          ",
            onclick: move |_| {
              trigger.set(Some((id.clone(), email())));
            },
            "Complete Registration"
          }
        }
      }
    )
}

fn base_input() -> &'static str {
    "
    width: 100%;
    padding: 0.75rem;
    margin-bottom: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
  "
}
