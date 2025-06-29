use crate::{use_event, ToastContext, ClientContext};
use dioxus::prelude::*;
const BUCKET_GOLF_SVG: Asset = asset!("/assets/bucket-golf.png");
use crate::Route;

#[component]
pub fn EventDetails(id: String) -> Element {
    let data = use_event(id,
      use_context::<Signal<ToastContext>>(),
      use_context::<Signal<ClientContext>>(),
    );

    match data.read_unchecked().as_ref() {
        Some(Some(event)) => rsx!(EventDetailsInner {
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
pub fn EventDetailsInner(
  id: String,
  name: String,
  location: String,
  date: String,
  image: String,
  banner: Option<String>,
  upsell: Option<String>,
) -> Element {
    rsx!(
      div {
        style: "
        min-height: 100vh;
        background-color: #f9fafb;
        padding: 3rem 1rem;
        font-family: sans-serif;
      ",
        div {
          style: "
          max-width: 48rem;
          margin: 0 auto;
          background: white;
          border-radius: 1rem;
          overflow: hidden;
          box-shadow: 0 4px 10px rgba(0,0,0,0.05);
        ",

          // Banner
          if let Some(banner_text) = banner.clone() {
              div {
                style: "
                background-color: #fde68a;
                color: #78350f;
                font-weight: bold;
                text-align: center;
                padding: 0.5rem;
              ",
                "{banner_text}"
              }
          }

          // Image thumbnail
          div {
            style: "padding: 1rem 2rem 0 2rem;",
            img {
              src: BUCKET_GOLF_SVG,
              alt: "Event image",
              style: "width: 100%; max-height: 200px; object-fit: cover; border-radius: 0.75rem;"
            }
          }

          // Main content
          div {
            style: "padding: 2rem;",

            h1 {
              style: "font-size: 2rem; font-weight: bold; color: #111827; margin-bottom: 0.5rem;",
              "{name}"
            }

            p {
              style: "color: #6b7280; margin-bottom: 0.75rem; font-size: 1rem;",
              "{date} · {location}"
            }

            if let Some(upsell_msg) = upsell.clone() {
                p {
                  style: "color: #dc2626; font-weight: 500; font-size: 0.95rem; margin-bottom: 1.25rem;",
                  "{upsell_msg}"
                }
            }

            // Description
            p {
              style: "font-size: 1rem; color: #374151; margin-bottom: 2rem; line-height: 1.6;",
              "Join fellow golf lovers for an afternoon of bucket golf at Quincy Park. All skill levels welcome! We'll be playing, mingling, and having a great time. Bring your clubs or borrow on-site. Snacks and drinks provided."
            }

            // Action buttons
            div {
              style: "display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 2rem;",
              Link {
                to: Route::RegisterEvent { id: id.clone() },
                button {
                  style: "
                  background-color: #4f46e5;
                  color: white;
                  font-weight: 600;
                  padding: 0.75rem 1.5rem;
                  border-radius: 0.5rem;
                  border: none;
                  cursor: pointer;
                ",
                  "Register Now"
                }
              }
              button {
                style: "
                background-color: #e0e7ff;
                color: #3730a3;
                font-weight: 500;
                padding: 0.75rem 1.5rem;
                border-radius: 0.5rem;
                border: none;
                cursor: pointer;
              ",
                "Share with Friends"
              }
            }

            // Additional details (optional)
            ul {
              style: "font-size: 0.95rem; color: #4b5563; list-style: disc; padding-left: 1.5rem;",
              li { "Bring your own putter if you have one." }
              li { "Arrive 15 minutes early for check-in." }
              li { "Light refreshments and water available on site." }
            }
          }
        }
      }
    )
}
