use std::sync::Arc;
use std::sync::Mutex;

use dioxus::prelude::*;
mod address_connector;
mod side;
mod storage;
use side::Dashboard;
use side::ProjectDetailPage;
use storage::LocalStorage;
use uuid::Uuid;
use web_sys::console;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");
const LOGO: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[nest("/cook-and-run")]
        #[route("/")]
        Dashboard {},
        #[route("/:id")]
        ProjectDetailPage { id: Uuid },
    #[end_nest]
    #[route("/:..route")]
    NotFound {
        route: Vec<String>,
    },
}
#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-4xl font-bold mb-4", "Welcome to the Traveling Cook Calculator!" }
            p { class: "text-lg mb-4",
                "This is a simple web application to help you plan your cooking and running events."
            }
            a {
                href: "/cook-and-run",
                class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600",
                "Get Started"
            }
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-4xl font-bold mb-4", "Welcome to the Traveling Cook Calculator!" }
            p { class: "text-lg mb-4", "NotFound" }
            p { class: "text-lg mb-4", "The requested route was not found: {route:?}" }
        }
    }
}

#[component]
fn App() -> Element {
    let storage = LocalStorage::new();
    if storage.is_err() {
        console::error_1(
            &format!(
                "Error when loading storage: {}",
                storage.err().expect("Expect storage error!")
            )
            .into(),
        );
        return error(
            "Fatal error!".to_string(),
            "Error when loading storage!".to_string(),
        );
    }

    let storage = storage.expect("Expected storage");
    let storage = Arc::new(Mutex::new(storage));
    use_context_provider(|| storage);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen bg-gray-50 flex flex-col",
            header { class: "bg-white shadow sticky top-0 z-50",
                div { class: "max-w-7xl mx-auto px-4 py-4 flex justify-between items-center",
                    a { href: "/cook-and-run",
                        img {
                            src: LOGO,
                            alt: "Traveling Cook Calculator",
                            class: "h-8 w-auto",
                        }
                        div { class: "flex items-center gap-4" }
                    }
                }
            }

            main { class: "flex h-full w-full", Router::<Route> {} }
        }
    }
}

fn error(headline: String, message: String) -> Element {
    rsx! {
        div {
            class: "flex p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400",
            role: "alert",
            svg {
                class: "shrink-0 inline w-4 h-4 me-3 mt-[2px]",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",

                view_box: "0 0 20 20",
                path { d: "M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z" }
            }
            span { class: "sr-only", "Danger" }
            div {
                span { class: "font-medium", "{headline}" }
                ul { class: "mt-1.5 list-disc list-insidemt-1.5 list-disc list-inside",
                    "{message}"
                }
            
            }
        }
    }
}
