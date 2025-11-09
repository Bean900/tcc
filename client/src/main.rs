mod address_connector;
pub mod auth0;
mod calculator;
mod side;
mod storage;

use dioxus::prelude::*;
use side::Dashboard;
use side::Overview;
use side::Teams;
use uuid::Uuid;
/*
use side::ProjectCalculationPage;
use side::ProjectCoursesPage;

use side::ProjectStartEndPage;
use side::ProjectTeamsPage;
use side::RunSchedule;
use side::ShareTeam;*/
use web_sys::console;
use web_sys::window;

pub use crate::auth0::AuthState;
use crate::side::Menu;
use crate::storage::StorageManager;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const PROVILE: Asset = asset!("/assets/profile.png");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");
const LOGO: Asset = asset!("/assets/logo.png");
fn main() {
    dioxus::launch(App);
}
/*    #[route("/:cook_and_run_id")]
#[route("/:cook_and_run_id/overview")]
    ProjectOverviewPage { cook_and_run_id: Uuid },
    #[route("/:cook_and_run_id/teams")]
    ProjectTeamsPage { cook_and_run_id: Uuid },
    #[route("/:cook_and_run_id/team-share/:share_id")]
    ShareTeam { cook_and_run_id: Uuid ,share_id: Uuid},
    #[route("/:cook_and_run_id/start-end")]
    ProjectStartEndPage { cook_and_run_id: Uuid },
    #[route("/:cook_and_run_id/courses")]
    ProjectCoursesPage { cook_and_run_id: Uuid },
    #[route("/:cook_and_run_id/calculation")]
    ProjectCalculationPage { cook_and_run_id: Uuid },
    #[route("/:cook_and_run_id/run-schedule/:team_id")]
    RunSchedule {cook_and_run_id:Uuid, team_id: Uuid },*/
#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[nest("/cook-and-run")]
            #[route("/")]
            Dashboard {},
            #[nest("/:cook_and_run_id")]
                #[layout(Menu)]
                    #[route("/overview")]
                    Overview {cook_and_run_id:Uuid},
                    #[route("/teams")]
                    Teams {cook_and_run_id:Uuid},
                  /*   #[route("/startend")]
                    StartEnd {cook_and_run_id:Uuid},
                    #[route("/courses")]
                    Courses {cook_and_run_id:Uuid},
                    #[route("/plan")]
                    Plan {cook_and_run_id:Uuid},*/
                #[end_layout]
            #[end_nest]
        #[end_nest]
    #[end_layout]
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
fn Wrapper() -> Element {
    let mut auth_signal = use_context_provider(|| Signal::new(AuthState::new()));

    use_effect(move || {
        let mut auth_signal = auth_signal.clone();
        if matches!(*auth_signal.read(), AuthState::Loading(_)) {
            let window = window().unwrap();
            let location = window.location();
            let search = location.search().unwrap();
            let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();

            let code = params.get("code");
            let state = params.get("state");

            spawn(async move {
                if let (Some(code), Some(state)) = (code, state) {
                    let auth = AuthState::new().callback(code, state).await;
                    auth_signal.set(auth);
                } else {
                    auth_signal.set(AuthState::LoggedOut);
                }
            });
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "min-h-screen flex flex-col bg-[#F8EFE1]",
            header { class: "shadow sticky top-0 z-50 bg-[#fdfaf6]",
                div { class: "max-w-7xl mx-auto px-4 py-4 flex justify-between items-center",
                    a { href: "/cook-and-run",
                        img {
                            src: LOGO,
                            alt: "Traveling Cook Calculator",
                            class: "h-8 w-auto",
                        }
                        div { class: "flex items-center gap-4" }
                    }
                    div { class: "flex items-center gap-4",
                        div { class: "relative",
                            match *auth_signal.read() {
                                AuthState::Loading(_) => rsx! {
                                    div { class: "relative flex items-center gap-2 bg-gray-200 px-3 py-2 rounded-full hover:bg-gray-300 focus:outline-none",
                                        "Loading..."
                                    }
                                },
                                AuthState::LoggedOut => rsx! {
                                    button {
                                        class: "relative flex items-center gap-2 bg-gray-200 px-3 py-2 rounded-full hover:bg-gray-300 focus:outline-none",
                                        onclick: move |_| {
                                            let path = use_route::<Route>();
                                            let (auth_state, auth_url) = AuthState::login(&path.to_string());
                                            auth_signal.set(auth_state);
                                            window().unwrap().location().set_href(&auth_url).unwrap();
                                        },
                                        "Login"
                                    }
                                },
                                AuthState::LoggedIn(_) => rsx! {
                                    button {
                                        class: "relative flex items-center gap-2 bg-gray-200 px-3 py-2 rounded-full hover:bg-gray-300 focus:outline-none",
                                        onclick: move |_| {
                                            let path = use_route::<Route>();
                                            let (auth_state, auth_url) = auth_signal.read().logout(&path.to_string());
                                            auth_signal.set(auth_state);
                                            window().unwrap().location().set_href(&auth_url).unwrap();
                                        },
                                        img { src: PROVILE, alt: "User Avatar", class: "h-8 w-8 rounded-full" }
                                        "Logout"
                                    }
                                },
                                AuthState::Error(_) => rsx! {
                                    button {
                                        class: "relative flex items-center gap-2 bg-gray-200 px-3 py-2 rounded-full hover:bg-gray-300 focus:outline-none",
                                        onclick: move |_| {
                                            let path = use_route::<Route>();
                                            let (auth_state, auth_url) = AuthState::login(&path.to_string());
                                            auth_signal.set(auth_state);
                                            window().unwrap().location().set_href(&auth_url).unwrap();
                                        },
                                        "Login..."
                                    }
                                },
                            }
                        
                        }
                    }
                }
            }
            main { class: "flex h-full w-full", Outlet::<Route> {} }
        }
    }
}

#[component]
fn App() -> Element {
    let storage = StorageManager::new();
    if let Err(s) = storage {
        console::error_1(&format!("Error when loading storage: {}", s).into());
        return error("Fatal error!", "Error when loading storage!");
    }
    let storage = storage.expect("Storage should be loaded correctly here");

    let _ = use_context_provider(|| Signal::new(storage));
    rsx! {
        Router::<Route> {}
    }
}

pub fn error(headline: &str, message: &str) -> Element {
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
