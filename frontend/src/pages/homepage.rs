use dioxus::prelude::*;
use crate::Route;
const HERO_SVG: Asset = asset!("/assets/hero-illustration.png");

#[component]
pub fn Homepage() -> Element {
    rsx!(
        div { style: "min-height: 100vh; background-color: #f9fafb; color: #1f2937; font-family: sans-serif;",
            // Nav
            nav { style: "display: flex; justify-content: space-between; align-items: center; padding: 1.5rem 2rem; background-color: white; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);",
                div { style: "font-size: 1.5rem; font-weight: bold; color: #EA3E4E;", "Konvo" }
                div { style: "display: flex; gap: 1.5rem; font-size: 0.875rem; font-weight: 500; color: #4b5563;",
                    a { href: "#features", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "Features" }
                    a { href: "#pricing", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "Pricing" }
                    a { href: "#about", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "About" }
                    a { href: "#contact", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "Contact" }
                }
            }

            // Hero Section
section {
  style: "
    display: flex;
    flex-wrap: nowrap;
    justify-content: space-between;
    align-items: center;
    padding: 6rem 2rem;
    background-color: #f9fafb;
    font-family: sans-serif;
    gap: 3rem;
  ",

  // Text Column
  div {
    style: "flex: 1 1 400px; max-width: 600px;",
    h1 {
      style: "font-size: 3rem; font-weight: 800; color: #111827; line-height: 1.2; letter-spacing: -0.5px;",
      "Build Stronger Communities with Konvo"
    }
    p {
      style: "margin-top: 1.25rem; font-size: 1.125rem; color: #6b7280; line-height: 1.75; max-width: 36rem;",
      "Konvo empowers leaders, teams, and organizers to launch events, grow membership, and bring people together — effortlessly and beautifully."
    }

    // CTA buttons
    div {
      style: "margin-top: 2rem; display: flex; gap: 1rem; flex-wrap: wrap;",
      Link {
        to: Route::GetStarted {},
        button {
          style: "
            background-color: #EA3E4E;
            color: white;
            font-weight: 600;
            padding: 0.75rem 1.5rem;
            border-radius: 0.5rem;
            border: none;
            cursor: pointer;
            font-size: 1rem;
            box-shadow: 0 4px 12px rgba(79, 70, 229, 0.3);
            transition: transform 0.2s ease;
          ",
          "onmouseover": "this.style.transform='scale(1.05)'",
          "onmouseout": "this.style.transform='scale(1)'",
          "Get Started"
        }
      }
      button {
        style: "
          background-color: white;
          color: #EA3E4E;
          border: 2px solid #EA3E4E;
          padding: 0.75rem 1.5rem;
          border-radius: 0.5rem;
          font-weight: 600;
          cursor: pointer;
          font-size: 1rem;
          transition: background-color 0.2s ease;
        ",
        "onmouseover": "this.style.backgroundColor='#eef2ff'",
        "onmouseout": "this.style.backgroundColor='white'",
        "Learn More"
      }
    }
  }

  // Image Column
  div {
    style: "flex: 1 1 400px; display: flex; justify-content: center;",
    img {
      src: HERO_SVG,  // Replace with your actual path
      alt: "Community Illustration",
      style: "max-width: 100%; height: auto; border-radius: 1rem; box-shadow: 0 10px 25px rgba(0,0,0,0.05);"
    }
  }
}
            // Features
            section {
              id: "features",
              style: "
                padding: 6rem 2rem;
                background-color: white;
                font-family: sans-serif;
                position: relative;
                overflow: hidden;
              ",
              // Section Title
              h2 {
                style: "
                  font-size: 2.25rem;
                  font-weight: 800;
                  text-align: center;
                  margin-bottom: 3rem;
                  color: #111827;
                ",
                "Unleash Your Community’s Full Potential"
              }

              // Feature Grid
              div {
                style: "
                  display: grid;
                  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
                  gap: 2.5rem;
                  text-align: left;
                  max-width: 80rem;
                  margin: 0 auto;
                ",
                
                // Feature 1
                div {
                  style: "
                    background: #f9fafb;
                    padding: 2rem;
                    border-radius: 1rem;
                    box-shadow: 0 2px 6px rgba(0,0,0,0.05);
                    transition: transform 0.3s ease;
                  ",
                  "onmouseover": "this.style.transform='scale(1.03)'",
                  "onmouseout": "this.style.transform='scale(1)'",
                  h3 { style: "font-size: 1.25rem; font-weight: 700; color: #EA3E4E;", "🎟️ Effortless Event Creation" }
                  p { style: "color: #374151; margin-top: 0.75rem; line-height: 1.6;",
                    "Host in-person and virtual events in seconds. Auto-invite your community, set RSVPs, and share with a link."
                  }
                },

                // Feature 2
                div {
                  style: "
                    background: #f9fafb;
                    padding: 2rem;
                    border-radius: 1rem;
                    box-shadow: 0 2px 6px rgba(0,0,0,0.05);
                    transition: transform 0.3s ease;
                  ",
                  "onmouseover": "this.style.transform='scale(1.03)'",
                  "onmouseout": "this.style.transform='scale(1)'",
                  h3 { style: "font-size: 1.25rem; font-weight: 700; color: #EA3E4E;", "📈 Deep Community Insights" }
                  p { style: "color: #374151; margin-top: 0.75rem; line-height: 1.6;",
                    "Visual dashboards show engagement trends, power users, and growth over time — no spreadsheet required."
                  }
                },

                // Feature 3
                div {
                  style: "
                    background: #f9fafb;
                    padding: 2rem;
                    border-radius: 1rem;
                    box-shadow: 0 2px 6px rgba(0,0,0,0.05);
                    transition: transform 0.3s ease;
                  ",
                  "onmouseover": "this.style.transform='scale(1.03)'",
                  "onmouseout": "this.style.transform='scale(1)'",
                  h3 { style: "font-size: 1.25rem; font-weight: 700; color: #EA3E4E;", "🤝 Real-Time Collaboration" }
                  p { style: "color: #374151; margin-top: 0.75rem; line-height: 1.6;",
                    "Build alongside teammates with shared access, role-based permissions, and live updates across your org."
                  }
                },

                // Feature 4 – Bonus Pizzazz
                div {
                  style: "
                    background: #f0fdf4;
                    padding: 2rem;
                    border-radius: 1rem;
                    box-shadow: inset 0 0 0 2px #bbf7d0;
                    transition: transform 0.3s ease;
                  ",
                  "onmouseover": "this.style.transform='scale(1.03)'",
                  "onmouseout": "this.style.transform='scale(1)'",
                  h3 { style: "font-size: 1.25rem; font-weight: 700; color: #059669;", "🧠 Smart Automations" }
                  p { style: "color: #065f46; margin-top: 0.75rem; line-height: 1.6;",
                    "Reminders, follow-ups, and scheduling — automated for you so you can focus on your community."
                  }
                }
              }
            }

            // Call to Action with Full Pizazz
            section {
              style: "
                position: relative;
                padding: 5rem 2rem;
                background: radial-gradient(circle at top left, #6366f1, #EA3E4E);
                color: white;
                text-align: center;
                font-family: sans-serif;
                overflow: hidden;
              ",
              // Sparkle background elements
              div {
                style: "
                  position: absolute;
                  top: -2rem;
                  left: -2rem;
                  width: 200%;
                  height: 200%;
                  background-image: url('/static/sparkles.svg'); /* or use a twinkly gif/png/svg */
                  background-repeat: repeat;
                  background-size: 400px;
                  animation: sparkle 20s linear infinite;
                  opacity: 0.05;
                  pointer-events: none;
                  z-index: 0;
                "
              }

              // Main Content
              div {
                style: "position: relative; z-index: 1; max-width: 40rem; margin: 0 auto;",
                h2 {
                  style: "
                    font-size: 2.75rem;
                    font-weight: 800;
                    margin-bottom: 1rem;
                    letter-spacing: -0.75px;
                    line-height: 1.2;
                    text-shadow: 0 2px 4px rgba(0,0,0,0.2);
                  ",
                  "Ready to Build a Movement?"
                }
                p {
                  style: "
                    font-size: 1.25rem;
                    color: #dbeafe;
                    margin-bottom: 2rem;
                    line-height: 1.7;
                  ",
                  "Turn quiet followers into raving superfans. Host events. Drive engagement. Become the pulse of your community."
                }
                Link {
                  to: Route::GetStarted {},
                button {
                  style: "
                    background: linear-gradient(to right, #ffffff, #e0e7ff);
                    color: #EA3E4E;
                    font-weight: 800;
                    font-size: 1.125rem;
                    padding: 0.85rem 2.25rem;
                    border: none;
                    border-radius: 9999px;
                    cursor: pointer;
                    box-shadow: 0 0 0 3px rgba(255,255,255,0.2);
                    transition: transform 0.2s ease, box-shadow 0.2s ease;
                  ",
                  "onmouseover": "this.style.transform='scale(1.07)'; this.style.boxShadow='0 8px 30px rgba(255,255,255,0.25)'",
                  "onmouseout": "this.style.transform='scale(1)'; this.style.boxShadow='0 0 0 3px rgba(255,255,255,0.2)'",
                  "Get Started Free"
                }}
              }
            }


            // Footer
            footer { style: "padding: 2rem; background-color: #f3f4f6; font-size: 0.875rem; color: #6b7280; text-align: center;",
                p { "© 2025 Konvo Inc. All rights reserved." }
            }
        }
        Outlet::<Route> {}
    )
}
