use dioxus::prelude::*;
use frontend::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");
const LOGO_SVG: Asset = asset!("/assets/logo.svg");
//const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone, Copy)]
pub struct AppContext {
    pub splash: bool,
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(AppContext {
        splash: true,
    }));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        //document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Splash() -> Element {
    rsx! {
        div { style: "
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                background-color: #0f172a;  /* change this to your brand color */
                display: flex;
                justify-content: center;
                align-items: center;
                flex-direction: column;
            ",
            // Logo (replace src with your actual logo)
            img {
                src: LOGO_SVG,
                alt: "Logo",
                style: "
                    width: 120px;
                    height: 120px;
                    margin-bottom: 16px;
                ",
            }
            // Optional tagline
            h1 { style: "
                    color: white;
                    font-size: 1.5rem;
                    font-family: sans-serif;
                    text-align: center;
                    margin: 0;
                ",
                "Crunching the numbers....."
            }
        }
    }
}

#[derive(Default)]
struct Agency {
    name: String,
    hwam: f32,
    vul: f32,
    swam: f32,
    csm: f32,
    fisma: f32,
    updated: &'static str,
}

#[derive(Props, PartialEq, Clone)]
pub struct MetricProps {
    label: &'static str,
    value: f32,
}
#[component]
pub fn Metric(props: MetricProps) -> Element {
    rsx! {
        div { style: "
            width: 5rem;
            padding: 1rem 0.3rem;
            background-color: #0f1116;
            border-radius: 12px;
            display: flex;
            flex-direction: column;
            align-items: center;
            text-align: center;
        ",

            span { style: "font-size: 0.75rem; opacity: 0.8;", "{props.label}" }
            span { style: "font-weight: bold; color: rgb(182,207,68)", "{props.value}" }
        }
    }
}

#[component]
pub fn Main() -> Element {
    //let context = use_context::<Signal<AppContext>>();
    //let on_click_agency = |_| {};
    let agencies = vec![
        Agency {
            name: "EXX".into(),
            hwam: 4.0,
            vul: 4.0,
            swam: 4.0,
            csm: 4.0,
            fisma: 4.0,
            updated: "Apr 8th, '25"
        },
        Agency {
            name: "ATR".into(),
            hwam: 4.0,
            vul: 4.0,
            swam: 4.0,
            csm: 4.0,
            fisma: 4.0,
            updated: "Apr 8th, '25"
        },
    ];
    
    rsx! {
        // Page wrapper
        div { style: "
                width: 100%;
                max-width: 1000px;
                min-width: 780px;
                min-height: 100vh;
                padding: 2rem;
                font-family: sans-serif;
                margin: 0 auto;
            ",

            // Header section
            header { style: "
                    margin-bottom: 2rem;
                    text-align: center;
                ",

                h1 { style: "
                        font-size: 2rem;
                        color:rgb(208, 208, 208);
                    ",
                    "Agency Metrics Dashboard"
                }
                p { style: "
                        color:rgb(147, 147, 147);
                    ",
                    "Overview of key performance indicators by agency"
                }
            }

            // Agency grid
            div { style: "
                    display: grid;
                    flex-direction: rows;
                    gap: 1.5rem;
                ",

                // Example agency card (you'll loop over data in real app)
                for agency in agencies.iter() {
                    div {
                        onclick: move |_| {}, // handler
                        style: "
                            background-color: rgb(82, 200, 207);
                            border-radius: 12px;
                            box-shadow: 0 2px 8px rgba(0,0,0,0.05);
                            padding: 0.8rem 2rem;
                            cursor: pointer;
                            transition: transform 0.2s;
                            display: flex;
                            justify-content: space-between;
                            align-items: center;
                            gap: 0.5rem;
                        ",
                        onmouseover: |_| {},
                        // Left side: agency name
                        div { style: "
                                display: flex;
                                align-items: center;
                                gap: 0.5rem;
                            ",
                            h2 { style: "
                                margin: 0;
                                font-size: 1.7rem;
                                color: #000000;
                                flex-shrink: 0;
                                font-weight: bold;
                            ",
                                "{agency.name}"
                            }
                            p { style: "
                                margin: 0;
                                font-size: 0.8rem;
                                color:rgb(67, 67, 67);
                                flex-shrink: 0;
                                font-weight: light;
                            ",
                                " - Updated {agency.updated}"
                            }
                        }
                        // Right side: metrics
                        div { style: "
                            display: flex;
                            gap: 0.5rem;
                            color: #ffffff;
                            font-size: 0.95rem;
                        ",
                            Metric { label: "HWAM", value: agency.hwam }
                            Metric { label: "VULN", value: agency.vul }
                            Metric { label: "SWAM", value: agency.swam }
                            Metric { label: "CSM", value: agency.csm }
                            Metric { label: "FISMA", value: agency.fisma }
                        }
                    }
                }
            }
        }
    }
}
