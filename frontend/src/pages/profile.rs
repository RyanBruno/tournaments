use dioxus::prelude::*;

#[component]
pub fn ProfilePage() -> Element {
  rsx!(
    div {
      style: "
        min-height: 100vh;
        background-color: #f9fafb;
        padding: 3rem 1rem;
        font-family: sans-serif;
        display: flex;
        justify-content: center;
      ",

      div {
        style: "
          display: grid;
          grid-template-columns: 1fr 2fr;
          gap: 2rem;
          max-width: 64rem;
          width: 100%;
        ",

        // Left Sidebar
        aside {
          style: "
            background: white;
            padding: 2rem;
            border-radius: 1rem;
            box-shadow: 0 2px 8px rgba(0,0,0,0.05);
            display: flex;
            flex-direction: column;
            align-items: center;
          ",

          img {
            src: "/static/profile-placeholder.png",
            alt: "Profile Picture",
            style: "width: 8rem; height: 8rem; border-radius: 9999px; object-fit: cover; border: 2px solid #e5e7eb; margin-bottom: 1rem;"
          },

          h2 { style: "font-size: 1.25rem; font-weight: 600; color: #111827;", "Jane Doe" },
          p { style: "font-size: 0.875rem; color: #6b7280;", "janedoe@example.com" },

          // Stats
          div {
            style: "margin-top: 2rem; width: 100%;",
            h3 { style: "font-size: 1rem; font-weight: 600; margin-bottom: 0.5rem;", "Stats" },
            ul {
              style: "font-size: 0.875rem; color: #374151; list-style: none; padding: 0;",
              li { "Total Events: 14" }
              li { "Wins: 3" }
              li { "First Event: June 2022" }
              li { "Hosted Events: 2" }
            }
          }
        }

        // Right Main Section
        main {
          style: "
            background: white;
            padding: 2rem;
            border-radius: 1rem;
            box-shadow: 0 2px 8px rgba(0,0,0,0.05);
          ",

          h1 { style: "font-size: 1.5rem; font-weight: bold; margin-bottom: 2rem;", "Profile Settings" },

          // Editable Fields
          form {
            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 2rem;",

            input { placeholder: "Full Name", value: "Jane Doe", style: base_input() }
            input { r#type: "email", placeholder: "Email", value: "janedoe@example.com", style: base_input() }
            input { placeholder: "Phone", value: "123-456-7890", style: base_input() }
            input { r#type: "url", placeholder: "Profile Picture URL", style: base_input() }
          }

          h2 { style: "font-size: 1.25rem; font-weight: 600; margin: 2rem 0 1rem;", "Billing Address" },
          form {
            style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 2rem;",
            input { placeholder: "Street", style: base_input() }
            input { placeholder: "City", style: base_input() }
            input { placeholder: "State", style: base_input() }
            input { placeholder: "Zip", style: base_input() }
          }

          h2 { style: "font-size: 1.25rem; font-weight: 600; margin: 2rem 0 1rem;", "Payments & Leagues" },
          ul {
            style: "font-size: 0.9rem; color: #374151; margin-bottom: 2rem;",
            li { "Last Payment: $23.80 – July 2024" }
            li { "Past Leagues: Summer 2023, Fall 2023" }
          }

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
            "Update Profile"
          }
        }
      }

      // Suggested Events (Full Width)
      div {
        style: "max-width: 64rem; margin: 3rem auto 0;",
        h2 { style: "font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem;", "Suggested Events" },
        ul {
          style: "display: flex; gap: 1rem; flex-wrap: wrap; list-style: none; padding: 0;",
          li {
            style: "flex: 1 1 18rem; background: white; padding: 1rem; border-radius: 0.75rem; box-shadow: 0 2px 6px rgba(0,0,0,0.05);",
            h3 { "Arlington Summer League" }
            p { "Saturday, July 27 – 3:00 PM" }
            a {
              href: "/event/arlington-summer",
              style: "display: inline-block; color: #4f46e5; margin-top: 0.5rem;",
              "View Event"
            }
          }
        }
      }
    }
  )
}

fn base_input() -> &'static str {
  "
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.95rem;
  "
}
