use dioxus::prelude::*;
use crate::Route;

pub fn Dashboard() -> Element {
  rsx!(
    div { style: "min-height: 100vh; background-color: #f9fafb; font-family: sans-serif; padding: 2rem;",
      div { style: "max-width: 60rem; margin: 0 auto;",
        // Header
        div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem;",
          h1 { style: "font-size: 2rem; font-weight: 700; color: #111827;", "Your Konvo HQ" }

          div { style: "display: flex; gap: 1rem; align-items: center;",
            Link {
              to: Route::CreateEvent {},
              button {
                style: "background-color: #4f46e5; color: white; font-weight: 600; padding: 0.5rem 1rem; border-radius: 0.375rem; border: none; cursor: pointer;",
                "＋ Create Event"
              }
            }
            a {
              href: "/profile",
              style: "display: inline-block; width: 2.5rem; height: 2.5rem; border-radius: 9999px; background-color: #e5e7eb; overflow: hidden; text-align: center; line-height: 2.5rem; font-weight: 600; color: #4f46e5; text-decoration: none;",
              "U"  // replace with user initial or image if available
            }
          }
        }

        // Announcements
        div {
          h2 { style: "font-size: 1.25rem; font-weight: 600; color: #111827; margin-bottom: 1rem;", "Announcements" }
          div { style: "background-color: #eef2ff; padding: 1rem; border-radius: 0.5rem; color: #1e3a8a;",
            p { "🎉 New dashboard analytics launching next week. Stay tuned!" }
          }
        }

        h2 {
          style: "text-align: left; font-size: 1.25rem; font-weight: 600; color: #111827; margin-bottom: 0rem;",
          "Create an event"
        }
        CreateEventForm {}

        // Events Section
        div { style: "margin-bottom: 3rem;",
          h2 { style: "font-size: 1.25rem; font-weight: 600; color: #111827; margin-bottom: 1rem;", "Your Upcoming Events" }
          div { style: "display: flex; flex-direction: column; gap: 1.5rem;",
            // Card 1
            div {
              style: "display: flex; gap: 1rem; background: white; border-radius: 0.75rem; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.05);",
              img {
                src: "/static/event1.jpg", // replace with your image path
                alt: "Launch Meetup",
                style: "width: 6rem; height: 6rem; object-fit: cover;"
              }
              div { style: "padding: 1rem;",
                h3 { style: "font-size: 1rem; font-weight: 600; color: #111827; margin-bottom: 0.25rem;", "Launch Meetup" }
                p { style: "font-size: 0.875rem; color: #6b7280;", "Monday, June 24 – 5:00 PM" }
                a {
                  href: "/event/launch-meetup", // adjust path as needed
                  style: "
                    display: inline-block;
                    font-size: 0.875rem;
                    font-weight: 500;
                    color: #4f46e5;
                    text-decoration: none;
                    padding: 0.5rem 1rem;
                    background-color: #eef2ff;
                    border-radius: 9999px;
                    transition: background-color 0.2s;
                  ",
                  "View Event"
                }
              }
            }
            // Card 2
            div {
              style: "display: flex; gap: 1rem; background: white; border-radius: 0.75rem; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.05);",
              img {
                src: "/static/event2.jpg",
                alt: "Planning Session",
                style: "width: 6rem; height: 6rem; object-fit: cover;"
              }
              div { style: "padding: 1rem;",
                h3 { style: "font-size: 1rem; font-weight: 600; color: #111827; margin-bottom: 0.25rem;", "Weekly Planning Session" }
                p { style: "font-size: 0.875rem; color: #6b7280;", "Wednesday, June 26 – 12:00 PM" }
                a {
                  href: "/event/launch-meetup", // adjust path as needed
                  style: "
                    display: inline-block;
                    font-size: 0.875rem;
                    font-weight: 500;
                    color: #4f46e5;
                    text-decoration: none;
                    padding: 0.5rem 1rem;
                    background-color: #eef2ff;
                    border-radius: 9999px;
                    transition: background-color 0.2s;
                  ",
                  "View Event"
                }
              }
            }
          }
        }

      }
    }
    // Footer
    footer { style: "padding: 2rem; background-color: #f3f4f6; font-size: 0.875rem; color: #6b7280; text-align: center;",
        p { "© 2025 Konvo Inc. All rights reserved." }
    }
  )
}

#[component]
pub fn CreateEventForm() -> Element {
  rsx!(
    div {
      style: "
        margin: 0.5rem auto;
        background-color: white;
        padding: 1.5rem;
        border-radius: 9999px;
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
          flex: 1;
          min-width: 10rem;
          padding: 0.75rem 1rem;
          border: none;
          background-color: #f3f4f6;
          border-radius: 9999px;
          font-size: 0.95rem;
        "
      }
      input {
        r#type: "datetime-local",
        style: "
          flex: 1;
          min-width: 12rem;
          padding: 0.75rem 1rem;
          border: none;
          background-color: #f3f4f6;
          border-radius: 9999px;
          font-size: 0.95rem;
        "
      }
      select {
        style: "
          flex: 1;
          min-width: 10rem;
          padding: 0.75rem 1rem;
          border: none;
          background-color: #f3f4f6;
          border-radius: 9999px;
          font-size: 0.95rem;
          color: #374151;
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
            background-color: #4f46e5;
            color: white;
            border: none;
            border-radius: 9999px;
            font-weight: 600;
            cursor: pointer;
            font-size: 0.95rem;
          ",
          "Create"
        }
      }
    }
  )
}
