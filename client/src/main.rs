use dioxus::{
    html::{area::alt, u::aria_hidden},
    prelude::*,
};

mod side;
mod storage;
use side::{Dashboard, ProjectDetailPage};
use storage::LocalStorage;
use web_sys::console;

const FAVICON: Asset = asset!("/assets/favicon.ico");
//const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");
const LOGO: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus::launch(App);
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

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // ProjectDashboard { storage }
        Main {}
    }
}

#[component]
pub fn Main() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 flex flex-col",
            header { class: "bg-white shadow sticky top-0 z-50",
                div { class: "max-w-7xl mx-auto px-4 py-4 flex justify-between items-center",
                    img {
                        src: LOGO,
                        alt: "Traveling Cook Calculator",
                        class: "h-8 w-auto",
                    }
                    div { class: "flex items-center gap-4" }
                }
            }

            main { class: "flex h-[calc(100vh-4rem)]",
                // Dashboard { breed: "Test" }
                ProjectDetailPage {}
            }
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
