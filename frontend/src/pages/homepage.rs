use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Homepage() -> Element {
    rsx!(
        div { style: "min-height: 100vh; background-color: #f9fafb; color: #1f2937; font-family: sans-serif;",
            // Nav
            nav { style: "display: flex; justify-content: space-between; align-items: center; padding: 1.5rem 2rem; background-color: white; box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);",
                div { style: "font-size: 1.5rem; font-weight: bold; color: #4f46e5;", "Konvo" }
                div { style: "display: flex; gap: 1.5rem; font-size: 0.875rem; font-weight: 500; color: #4b5563;",
                    a { href: "#features", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "Features" }
                    a { href: "#pricing", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "Pricing" }
                    a { href: "#about", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "About" }
                    a { href: "#contact", style: "text-decoration: none; color: inherit; transition: color 0.3s;", onmouseover: |_| {}, "Contact" }
                }
            }

            // Hero
            section { style: "padding: 5rem 2rem; text-align: center;",
                h1 { style: "font-size: 3rem; font-weight: bold; color: #111827; line-height: 1.2;", "Build Better Communities" }
                p { style: "margin-top: 1rem; font-size: 1.125rem; color: #6b7280; max-width: 40rem; margin-left: auto; margin-right: auto;",
                    "Konvo helps teams and individuals connect around shared experiences. Organize, share, and grow your community effortlessly."
                }
                div { style: "margin-top: 2rem; display: flex; justify-content: center; gap: 1rem;",
                    Link { to: Route::GetStarted {},
                      button { style: "background-color: #4f46e5; color: white; padding: 0.75rem 1.5rem; border-radius: 0.5rem; box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05); border: none; cursor: pointer;", "Get Started" }
                    }
                    button { style: "border: 1px solid #4f46e5; color: #4f46e5; padding: 0.75rem 1.5rem; border-radius: 0.5rem; background: transparent; cursor: pointer;", "Learn More" }
                }
            }

            // Features
            section { id: "features", style: "padding: 4rem 2rem; background-color: white;",
                h2 { style: "font-size: 2rem; font-weight: bold; text-align: center; margin-bottom: 3rem;", "Powerful Features" }
                div { style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; text-align: center;",
                    div {
                        h3 { style: "font-size: 1.25rem; font-weight: 600; color: #4f46e5;", "Event Management" }
                        p { style: "color: #6b7280; margin-top: 0.5rem;", "Easily create, share, and track your events with our intuitive dashboard." }
                    }
                    div {
                        h3 { style: "font-size: 1.25rem; font-weight: 600; color: #4f46e5;", "Community Insights" }
                        p { style: "color: #6b7280; margin-top: 0.5rem;", "Track engagement and growth with built-in analytics." }
                    }
                    div {
                        h3 { style: "font-size: 1.25rem; font-weight: 600; color: #4f46e5;", "Seamless Collaboration" }
                        p { style: "color: #6b7280; margin-top: 0.5rem;", "Work with team members in real time, assign tasks, and share resources." }
                    }
                }
            }

            // Call to action
            section { style: "padding: 4rem 2rem; background-color: #4f46e5; color: white; text-align: center;",
                h2 { style: "font-size: 2rem; font-weight: bold;", "Ready to grow your community?" }
                p { style: "margin-top: 0.5rem; font-size: 1.125rem;", "Join thousands of others using Konvo to create lasting impact." }
                button { style: "margin-top: 1.5rem; background-color: white; color: #4f46e5; font-weight: 600; padding: 0.75rem 1.5rem; border-radius: 0.5rem; border: none; cursor: pointer;", "Get Started Free" }
            }

            // Footer
            footer { style: "padding: 2rem; background-color: #f3f4f6; font-size: 0.875rem; color: #6b7280; text-align: center;",
                p { "© 2025 Konvo Inc. All rights reserved." }
            }
        }
        Outlet::<Route> {}
    )
}
