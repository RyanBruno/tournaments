use dioxus::prelude::*;

#[component]
pub fn NotificationsDropdown() -> Element {
  let mut show = use_signal(|| false);

  let mut notifications = vec![
    ("⛳️", "Bucket Golf League starts soon!"),
    ("🏋️", "Lifting Club session confirmed."),
    ("🗓️", "New event: Community Yoga – July 18"),
  ];
  notifications.clear();

  rsx!(
    div {
      style: "position: relative;",
      
      // Bell button
      if notifications.len() > 0 {
      button {
        onclick: move |_| *show.write() = !show(),
        style: "
          position: relative;
          background: none;
          border: none;
          cursor: pointer;
          font-size: 1.5rem;
        ",
        "🔔",
          span {
            style: "
              position: absolute;
              top: -0.25rem;
              right: -0.25rem;
              background: red;
              color: white;
              font-size: 0.75rem;
              width: 1rem;
              height: 1rem;
              border-radius: 9999px;
              display: flex;
              align-items: center;
              justify-content: center;
            ",
            "{notifications.len()}"
          }
        }
      }

      // Dropdown
      if show() {
        div {
          style: "
            position: absolute;
            right: 0;
            top: 2.5rem;
            background: white;
            border: 1px solid #e5e7eb;
            border-radius: 0.75rem;
            box-shadow: 0 6px 16px rgba(0,0,0,0.1);
            width: 18rem;
            z-index: 50;
            padding: 1rem;
            font-family: sans-serif;
          ",

          h4 {
            style: "font-size: 1rem; font-weight: 600; margin-bottom: 0.75rem; color: #111827;",
            "Notifications"
          }

          ul {
            style: "list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 0.75rem;",
            for notification in notifications.iter() {
              li {
                style: "
                  display: flex;
                  align-items: center;
                  gap: 0.5rem;
                  padding: 0.5rem;
                  background-color: #f9fafb;
                  border-radius: 0.5rem;
                  font-size: 0.9rem;
                  color: #374151;
                ",
                span { style: "font-size: 1.25rem;", "{notification.0}" }
                span { "{notification.1}" }
              }
            }
          }

          /*a {
            href: "/events",
            style: "
              display: block;
              text-align: right;
              font-size: 0.875rem;
              color: #4f46e5;
              font-weight: 500;
              margin-top: 0.75rem;
              text-decoration: none;
            ",
            "View All Events"
          }*/
        }
      }
    }
  )
}
