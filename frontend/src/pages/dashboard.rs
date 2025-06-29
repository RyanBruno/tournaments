use crate::{use_dashboard_api, DashboardApi, Event, Route, ToastContext, ClientContext};
use dioxus::prelude::*;
const BUCKET_GOLF_SVG: Asset = asset!("/assets/bucket-golf.png");

#[component]
pub fn Dashboard() -> Element {
    let toast = use_context::<Signal<ToastContext>>();
    let client = use_context::<Signal<ClientContext>>();
    let dashboard_data = use_dashboard_api(toast, client);

    match dashboard_data.read_unchecked().as_ref() {
        Some(Some(data)) => rsx!(DashboardHelper {
            dashboard_data: data.clone()
        }),
        _ => rsx!(),
    }
}
#[component]
pub fn DashboardHelper(dashboard_data: DashboardApi) -> Element {
    rsx!(DashboardLayout {
      name: dashboard_data.name,
      announcment: dashboard_data.announcment,
      events: dashboard_data.events,
    })
}

#[component]
pub fn DashboardLayout(name: String, announcment: String, events: Vec<Event>) -> Element {
    rsx!(
      div { style: "min-height: 100vh; background-color: #f9fafb; font-family: sans-serif; padding: 2rem;",
        div { style: "max-width: 60rem; margin: 0 auto;",
          // Header
          div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;",
            h1 { style: "font-size: 2rem; font-weight: 700; color: #111827;", "{name}" }

            div { style: "display: flex; gap: 1rem; align-items: center;",
              Link {
                to: Route::CreateEvent {},
                button {
                  style: "background-color: #EA3E4E; color: white; font-weight: 600; padding: 0.5rem 1rem; border-radius: 0.375rem; border: none; cursor: pointer;",
                  "＋ Create Event"
                }
              }
              a {
                href: "/profile",
                style: "display: inline-block; width: 2.5rem; height: 2.5rem; border-radius: 9999px; background-color: #e5e7eb; overflow: hidden; text-align: center; line-height: 2.5rem; font-weight: 600; color: #EA3E4E; text-decoration: none;",
                "U"  // replace with user initial or image if available
              }
            }
          }

          // Announcements
          div {
            h2 { style: "font-size: 1.25rem; font-weight: 600; color: #111827; margin-bottom: 1rem;", "Announcements" }
            div { style: "background-color: #eef2ff; padding: 1rem; border-radius: 0.5rem; color: #1e3a8a;",
              p { "{announcment}" }
              //p { "⛳️ New summer leagues of bucket golf just dropped. Rally your crew and start swinging!" }

            }
          }

          h2 {
            style: "text-align: left; font-size: 1.25rem; font-weight: 600; color: #111827; margin-bottom: 0rem;",
            "Create an event"
          }
          CreateEventForm {}

          // Events Section
          div {
            style: "margin-bottom: 3rem; font-family: sans-serif;",

            // Header Row: Title + Dropdown
            div {
              style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem; flex-wrap: wrap; gap: 1rem;",

              h2 {
                style: "font-size: 1.25rem; font-weight: 600; color: #111827;",
                "Your Upcoming Events"
              }

              select {
                style: "
                padding: 0.5rem 1rem;
                font-size: 0.875rem;
                border: 1px solid #d1d5db;
                border-radius: 9999px;
                background-color: #f9fafb;
                color: #374151;
                cursor: pointer;
              ",
                option { value: "", disabled: true, selected: true, "Filter by city" }
                option { value: "arlington", "Arlington" }
                option { value: "dc", "Washington, DC" }
                option { value: "baltimore", "Baltimore" }
                option { value: "richmond", "Richmond" }
              }
            }
            div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
              for event in events.iter() {
                  EventCard {
                    key: "{event.id}",
                    event: event.clone()
                  }

              }
          }}
        }
      }
      // Footer
      footer { style: "padding: 2rem; background-color: #f3f4f6; font-size: 0.875rem; color: #6b7280; text-align: center;",
        p { "© 2025 Konvo Inc. All rights reserved." }
      }
    )
}

#[component]
pub fn EventCard(event: Event) -> Element {
    rsx!(
      div {
        style: "
        position: relative;
        display: flex;
        gap: 1rem;
        background: white;
        border-radius: 0.75rem;
        overflow: hidden;
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
        align-items: stretch;
      ",
        // Flash banner
        if event.banner.is_some() {
            div {
            style: "
            position: absolute;
            top: 0;
            left: 0;
            background-color: #facc15;
            color: #78350f;
            font-weight: 700;
            font-size: 0.75rem;
            padding: 0.25rem 0.75rem;
            border-bottom-right-radius: 0.5rem;
            z-index: 1;
          ",
            "{event.banner.clone().unwrap()}"
          }
        }
        img {
          src: BUCKET_GOLF_SVG,
          alt: "Bucket Golf League",
          style: "
          width: 8rem;
          object-fit: cover;
          display: block;
          align-self: stretch;
          flex-shrink: 0;
        "
        }
        div {
          style: "padding: 1rem; display: flex; flex-direction: column; justify-content: center;",
          h3 {
            style: "font-size: 1rem; font-weight: 600; color: #111827; margin-bottom: 0.25rem;",
            {event.name.clone()}
          }
          p {
            style: "font-size: 0.875rem; color: #6b7280;",
            "{event.date.clone()} - {event.location.clone()}"
          }
          if event.upsell.is_some() {
            p {
              style: "font-size: 0.875rem; color: #dc2626; margin-top: 0.25rem;",
              "{event.upsell.clone().unwrap()}"
            }
          }
          Link {
            to: Route::EventDetails { id: event.id.clone() },
            span {
              style: "
              display: inline-block;
              font-size: 0.875rem;
              font-weight: 500;
              color: #EA3E4E;
              text-decoration: none;
              padding: 0.5rem 1rem;
              background-color: #eef2ff;
              border-radius: 9999px;
              margin-top: 0.75rem;
              transition: background-color 0.2s;
              flex-shrink: 0;
              align-self: start;
            ",
              "View Event"
            }
          }
        }
      }
    )
}

#[component]
pub fn CreateEventForm() -> Element {
    rsx!(
      div {
        class: "event-form",
        style: "
        margin: 0.5rem auto;
        background-color: white;
        padding: 1.5rem;
        border-radius: 2rem;
        box-shadow: 0 2px 6px rgba(0,0,0,0.05);
        max-width: 100%;
        width: 100%;
        display: flex;
        gap: 1rem;
        align-items: center;
        justify-content: center;
        font-family: sans-serif;
        flex-wrap: wrap;
      ",
        input {
          r#type: "text",
          placeholder: "Event location",
          style: "
          min-width: 10rem;
          padding: 0.75rem 1rem;
          border: none;
          background-color: #f3f4f6;
          border-radius: 9999px;
          font-size: 0.95rem;
          width: 100%;
        "
        }
        input {
          r#type: "datetime-local",
          style: "
          min-width: 12rem;
          padding: 0.75rem 1rem;
          border: none;
          background-color: #f3f4f6;
          border-radius: 9999px;
          font-size: 0.95rem;
          width: 100%;
        "
        }
        select {
          style: "
          min-width: 10rem;
          padding: 0.75rem 1rem;
          border: none;
          background-color: #f3f4f6;
          border-radius: 9999px;
          font-size: 0.95rem;
          color: #374151;
          width: 100%;
        ",
          option { value: "", disabled: true, selected: true, "Sport" }
          option { value: "basketball", "Basketball" }
          option { value: "soccer", "Soccer" }
          option { value: "lifting", "Lifting" }
          option { value: "running", "Running" }
          option { value: "yoga", "Yoga" }
        }
        Link {
          to: Route::CreateEvent {},
          button {
            r#type: "submit",
            style: "
            padding: 0.75rem 1.5rem;
            background-color: #EA3E4E;
            color: white;
            border: none;
            border-radius: 9999px;
            font-weight: 600;
            cursor: pointer;
            font-size: 0.95rem;
            width: 100%;
          ",
            "Create"
          }
        }
      }
    )
}
