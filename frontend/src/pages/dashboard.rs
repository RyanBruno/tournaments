use crate::{use_dashboard_api, ClientContext, NotificationsDropdown, Route, ToastContext,
  BrandContext,
};
use dioxus::prelude::*;
use models::{DashboardView, Event};
const BUCKET_GOLF_SVG: Asset = asset!("/assets/bucket-golf.png");

#[component]
pub fn Dashboard() -> Element {
    let toast = use_context::<Signal<ToastContext>>();
    let client = use_context::<Signal<ClientContext>>();
    let dashboard_data = use_dashboard_api(toast, client);

    match dashboard_data.read_unchecked().as_ref() {
        Some(Some(data)) => rsx!(
          DashboardHelper { dashboard_data: data.clone() }
        ),
        _ => rsx!(),
    }
}
#[component]
pub fn DashboardHelper(dashboard_data: DashboardView) -> Element {
    rsx!(
      DashboardLayout {
        name: dashboard_data.name,
        announcement: dashboard_data.announcement,
        events: dashboard_data.events,
        active_events: dashboard_data.active_events,
      }
    )
}

#[component]
pub fn DashboardLayout(name: String, announcement: String, events: Vec<Event>, active_events: Vec<Event>) -> Element {
    let brand = use_context::<Signal<BrandContext>>();
    let BrandContext {name: _, logo: _, primary_color, secondary_color} = brand.read().clone();
    rsx!(
      div { style: "min-height: 100vh; background: linear-gradient(135deg, #f9fafb 60%, #e0e7ff 100%); font-family: 'Inter', sans-serif; padding: 0;",
        // Hero/Header
        div { style: "background: linear-gradient(90deg, {primary_color} 0%, {secondary_color} 100%); padding: 2.5rem 2rem 2rem 2rem; margin-bottom: 2rem; box-shadow: 0 4px 24px rgba(234,62,78,0.07);",
          div { style: "max-width: 60rem; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;",
            h1 { style: "font-size: 2.5rem; font-weight: 800; color: white; letter-spacing: -0.03em;",
              "{name}"
            }
            div { style: "display: flex; gap: 1.25rem; align-items: center;",
              Link { to: Route::CreateEvent {},
                button { style: "background: white; color: {secondary_color}; font-weight: 700; padding: 0.6rem 1.4rem; border-radius: 0.5rem; border: none; box-shadow: 0 2px 8px rgba(0,0,0,0.04); font-size: 1rem; transition: background 0.2s; cursor: pointer;",
                  "＋ Create Event"
                }
              }
              NotificationsDropdown {}
              Link { to: Route::ProfilePage {},
                span { style: "display: inline-block; width: 2.7rem; height: 2.7rem; border-radius: 9999px; background: linear-gradient(135deg, #e0e7ff 60%, {primary_color} 100%); overflow: hidden; text-align: center; line-height: 2.7rem; font-weight: 700; color: {secondary_color}; font-size: 1.2rem; border: 2px solid #fff; box-shadow: 0 2px 8px rgba(0,0,0,0.04);",
                  "U"
                }
              }
            }
          }
        }
        div { style: "padding: 0 2rem",

          // Announcements
          div { style: "max-width: 60rem; margin: 0 auto 2rem auto;",
            div { style: "background: linear-gradient(90deg, #eef2ff 80%, #e0e7ff 100%); padding: 1.5rem 2rem; border-radius: 1rem; color: #1e3a8a; box-shadow: 0 2px 8px rgba(30,58,138,0.04); margin-bottom: 2rem; display: flex; align-items: center; gap: 1rem;",
              div {
                h2 { style: "font-size: 1.2rem; font-weight: 700; color: #1e3a8a; margin-bottom: 0.25rem;",
                  "Announcements"
                }
                p { style: "font-size: 1rem; color: #374151;", "{announcement}" }
              }
            }
          }

          // Create Event Section
          div { style: "max-width: 60rem; margin: 0 auto 2.5rem auto;",
            h2 { style: "font-size: 1.25rem; font-weight: 700; color: {primary_color}; margin-bottom: 0.5rem; letter-spacing: -0.01em;",
              "Create an event"
            }
            CreateEventForm {}
          }
          ActiveEvents { events: active_events }

          // Events Section
          div { style: "max-width: 60rem; margin: 0 auto 3rem auto;",
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; flex-wrap: wrap; gap: 1rem;",
              h2 { style: "font-size: 1.35rem; font-weight: 700; color: #111827;",
                "Upcoming Events"
              }
              select { style: "
                padding: 0.5rem 1.2rem;
                font-size: 1rem;
                border: 1px solid #d1d5db;
                border-radius: 9999px;
                background: #f9fafb;
                color: #374151;
                cursor: pointer;
                box-shadow: 0 1px 3px rgba(0,0,0,0.03);
              ",
                option {
                  value: "",
                  disabled: true,
                  selected: true,
                  "Filter by city"
                }
                option { value: "arlington", "Arlington" }
                option { value: "dc", "Washington, DC" }
                option { value: "baltimore", "Baltimore" }
                option { value: "richmond", "Richmond" }
              }
            }
            div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 2rem;",
              for event in events.iter() {
                EventCard { key: "{event.id}", event: event.clone() }
              }
            }
          }
        }

        // Footer
        footer { style: "padding: 2rem; background: #f3f4f6; font-size: 0.95rem; color: #6b7280; text-align: center; border-top: 1px solid #e5e7eb;",
          p { "© 2025 Konvo Inc. All rights reserved." }
        }
      }
    )
}

#[component]
pub fn EventCard(event: Event) -> Element {
    let brand = use_context::<Signal<BrandContext>>();
    let BrandContext {name: _, logo: _, primary_color, secondary_color} = brand.read().clone();
    rsx!(
      div { style: "
          position: relative;
          display: flex;
          flex-direction: column;
          background: white;
          border-radius: 1.25rem;
          overflow: hidden;
          box-shadow: 0 4px 16px rgba(0,0,0,0.07);
          transition: box-shadow 0.2s, transform 0.2s;
          align-items: stretch;
          min-height: 16rem;
          border: 1px solid #e5e7eb;
        ",
        // Flash banner
        if event.banner.is_some() {
          div { style: "
              position: absolute;
              top: 0;
              left: 0;
              background: linear-gradient(90deg, #facc15 80%, #fbbf24 100%);
              color: #78350f;
              font-weight: 700;
              font-size: 0.85rem;
              padding: 0.4rem 1rem;
              border-bottom-right-radius: 0.75rem;
              z-index: 1;
              box-shadow: 0 2px 8px rgba(250,204,21,0.07);
            ",
            "{event.banner.clone().unwrap()}"
          }
        }
        img {
          src: BUCKET_GOLF_SVG,
          alt: "Bucket Golf League",
          style: "
            width: 100%;
            height: 8rem;
            object-fit: cover;
            display: block;
            border-top-left-radius: 1.25rem;
            border-top-right-radius: 1.25rem;
            background: #f3f4f6;
          ",
        }
        div { style: "padding: 1.2rem 1.5rem; display: flex; flex-direction: column; justify-content: center; flex: 1;",
          h3 { style: "font-size: 1.15rem; font-weight: 700; color: #111827; margin-bottom: 0.4rem;",
            "{event.name.clone()}"
          }
          p { style: "font-size: 0.97rem; color: #6b7280; margin-bottom: 0.3rem;",
            "{event.date.clone()} · {event.location.clone()}"
          }
          if event.upsell.is_some() {
            p { style: "font-size: 0.97rem; color: #dc2626; margin-bottom: 0.3rem;",
              "{event.upsell.clone().unwrap()}"
            }
          }
          Link {
            to: Route::EventDetails {
                id: event.id.clone(),
            },
            span { style: "
                display: inline-block;
                font-size: 0.97rem;
                font-weight: 600;
                color: #fff;
                background: linear-gradient(90deg, {primary_color} 00%, {secondary_color} 100%);
                text-decoration: none;
                padding: 0.6rem 1.2rem;
                border-radius: 9999px;
                margin-top: 0.7rem;
                transition: background 0.2s;
                align-self: start;
                box-shadow: 0 1px 4px rgba(234,62,78,0.07);
                border: none;
                cursor: pointer;
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
    let brand = use_context::<Signal<BrandContext>>();
    let BrandContext {name: _, logo: _, primary_color, secondary_color: _} = brand.read().clone();
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
        ",
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
        ",
        }
        select { style: "
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
        Link { to: Route::CreateEvent {},
          button {
            r#type: "submit",
            style: "
            padding: 0.75rem 1.5rem;
            background-color: {primary_color};
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

#[component]
pub fn ActiveEvents(events: Vec<Event>) -> Element {
    if events.is_empty() {
        return rsx!();
    }

    rsx!(
      section { style: "
        padding: 1rem 0;
        margin-bottom: 3rem;
        font-family: sans-serif;
      ",
        h2 { style: "
          font-size: 1.5rem;
          font-weight: bold;
          color: #1f2937;
          text-align: left;
          margin-bottom: 1rem;
        ",
          "Active Events"
        }
        ul { style: "
          display: flex;
          flex-direction: column;
          gap: 1rem;
          padding: 0;
          margin: 0;
          list-style: none;
        ",
          for event in events.iter() {
            li { style: "
              background: white;
              border-radius: 0.75rem;
              box-shadow: 0 1px 3px rgba(0,0,0,0.05);
              padding: 1rem 1.25rem;
              display: flex;
              flex-direction: row;
              align-items: center;
              justify-content: space-between;
              width: 100%;
            ",
              div { style: "flex-grow: 1;",
                h3 { style: "font-size: 1.125rem; font-weight: 600; color: #111827; margin-bottom: 0.25rem;",
                  "{event.name}"
                }
                p { style: "font-size: 0.875rem; color: #6b7280;",
                  "{event.date} · {event.location}"
                }
              }
              a {
                href: format!("/event/{}", event.id),
                style: "
                font-size: 0.875rem;
                font-weight: 500;
                color: #4f46e5;
                background-color: #eef2ff;
                padding: 0.5rem 1rem;
                border-radius: 9999px;
                text-decoration: none;
                transition: background-color 0.2s ease;
              ",
                "View Details"
              }
            }
          }
        }
      }
    )
}
