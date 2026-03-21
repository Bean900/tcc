mod address_connector;
pub mod auth0;
mod calculator;
mod side;
mod storage;
use dioxus::prelude::*;
use side::Calculate;
use side::Callback;
use side::Courses;
use side::Dashboard;
use side::Overview;
use side::Plan;
use side::StartEnd;
use side::Teams;
use uuid::Uuid;

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

// ─────────────────────────────────────────────
//  Routing
// ─────────────────────────────────────────────

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/callback?:code&:state")]
        Callback { code: String, state: String },
        #[nest("/cook-and-run")]
            #[route("/")]
            Dashboard {},
            #[nest("/:cook_and_run_id")]
                #[layout(Menu)]
                    #[route("/overview")]
                    Overview { cook_and_run_id: Uuid },
                    #[route("/teams")]
                    Teams { cook_and_run_id: Uuid },
                    #[route("/startend")]
                    StartEnd { cook_and_run_id: Uuid },
                    #[route("/courses")]
                    Courses { cook_and_run_id: Uuid },
                    #[route("/calculate")]
                    Calculate { cook_and_run_id: Uuid },
                    #[route("/plan/:team_id")]
                    Plan { cook_and_run_id: Uuid, team_id: Uuid },
                #[end_layout]
            #[end_nest]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

impl Route {
    fn to_string(&self) -> String {
        match self {
            Route::Dashboard {} => "cook-and-run".to_string(),
            Route::Overview { cook_and_run_id } => format!("overview.{}", cook_and_run_id),
            Route::Teams { cook_and_run_id } => format!("teams.{}", cook_and_run_id),
            Route::StartEnd { cook_and_run_id } => format!("startend.{}", cook_and_run_id),
            Route::Courses { cook_and_run_id } => format!("courses.{}", cook_and_run_id),
            Route::Calculate { cook_and_run_id } => format!("calculate.{}", cook_and_run_id),
            _ => "-".to_string(),
        }
    }

    fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split('.').collect();
        match parts[0] {
            "cook-and-run" => Route::Dashboard {},
            "overview" => {
                if parts.len() == 2 {
                    if let Ok(uuid) = Uuid::parse_str(parts[1]) {
                        return Route::Overview {
                            cook_and_run_id: uuid,
                        };
                    }
                }
                Route::NotFound {
                    route: vec![s.to_string()],
                }
            }
            "teams" => {
                if parts.len() == 2 {
                    if let Ok(uuid) = Uuid::parse_str(parts[1]) {
                        return Route::Teams {
                            cook_and_run_id: uuid,
                        };
                    }
                }
                Route::NotFound {
                    route: vec![s.to_string()],
                }
            }
            "startend" => {
                if parts.len() == 2 {
                    if let Ok(uuid) = Uuid::parse_str(parts[1]) {
                        return Route::StartEnd {
                            cook_and_run_id: uuid,
                        };
                    }
                }
                Route::NotFound {
                    route: vec![s.to_string()],
                }
            }
            "courses" => {
                if parts.len() == 2 {
                    if let Ok(uuid) = Uuid::parse_str(parts[1]) {
                        return Route::Courses {
                            cook_and_run_id: uuid,
                        };
                    }
                }
                Route::NotFound {
                    route: vec![s.to_string()],
                }
            }
            "calculate" => {
                if parts.len() == 2 {
                    if let Ok(uuid) = Uuid::parse_str(parts[1]) {
                        return Route::Calculate {
                            cook_and_run_id: uuid,
                        };
                    }
                }
                Route::NotFound {
                    route: vec![s.to_string()],
                }
            }
            _ => Route::NotFound {
                route: vec![s.to_string()],
            },
        }
    }
}

// ─────────────────────────────────────────────
//  Home page
// ─────────────────────────────────────────────

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen px-6",

            // Logo / title block
            div { class: "text-center mb-10",
                p { class: "text-[11px] font-semibold tracking-[0.15em] uppercase text-amber-600 mb-2",
                    "Cook & Run"
                }
                h1 { class: "text-4xl font-bold text-zinc-900 mb-3",
                    "Traveling Cook Calculator"
                }
                p { class: "text-base text-zinc-500 max-w-sm mx-auto leading-relaxed",
                    "Plan your cooking and running events with ease."
                }
            }

            // CTA card
            div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm px-8 py-6 flex flex-col items-center gap-4",
                a {
                    href: "/cook-and-run",
                    class: "bg-[#D67229] hover:bg-[#C66741] text-white \
                            rounded-xl px-6 py-2.5 text-sm font-medium \
                            transition-colors duration-150 cursor-pointer",
                    "Get Started"
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Not found page
// ─────────────────────────────────────────────

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen px-6",
            div { class: "w-full max-w-sm text-center",

                // 404 card
                div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden",
                    div { class: "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center justify-center gap-2.5",
                        div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                        span { class: "text-sm font-semibold text-zinc-800", "Page not found" }
                    }
                    div { class: "px-6 py-8 space-y-4",
                        p { class: "text-sm text-zinc-500",
                            "The page you're looking for doesn't exist."
                        }
                        p { class: "text-xs text-zinc-400 font-mono bg-zinc-50 \
                                    rounded-lg px-3 py-1.5 border border-zinc-100",
                            "{route:?}"
                        }
                        a {
                            href: "/cook-and-run",
                            class: "inline-block bg-[#D67229] hover:bg-[#C66741] text-white \
                                    rounded-xl px-5 py-2 text-sm font-medium \
                                    transition-colors duration-150",
                            "Back to Dashboard"
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  App shell / Wrapper
// ─────────────────────────────────────────────

#[component]
fn Wrapper() -> Element {
    let mut storage_signal = use_context::<Signal<StorageManager>>();

    // Shared auth button base class
    const AUTH_BTN: &str = "flex items-center gap-2 px-3 py-1.5 rounded-xl text-sm font-medium \
         text-zinc-600 bg-amber-50 border border-amber-200 \
         hover:bg-amber-100 hover:border-amber-300 \
         transition-colors duration-150 cursor-pointer";

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "min-h-screen flex flex-col bg-[#F8EFE1]",

            // ── Header ────────────────────────────────────────────
            header { class: "sticky top-0 z-50 bg-[#FDFAF6] border-b border-amber-200/60 shadow-sm",
                div { class: "max-w-7xl mx-auto px-6 py-3 flex justify-between items-center",

                    // Logo
                    a { href: "/cook-and-run", class: "flex items-center gap-3",
                        img {
                            src: LOGO,
                            alt: "Cook & Run",
                            class: "h-8 w-auto",
                        }
                    }

                    // Auth button
                    div { class: "flex items-center gap-3",
                        match storage_signal.read().get_auth_state() {
                            AuthState::Loading(_) => rsx! {
                                button {
                                    class: "{AUTH_BTN} opacity-60 cursor-not-allowed",
                                    disabled: true,
                                    // Mini amber spinner
                                    svg {
                                        class: "w-3.5 h-3.5 animate-spin text-amber-400",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        circle {
                                            cx: "12", cy: "12", r: "10",
                                            stroke: "currentColor",
                                            stroke_width: "3",
                                            stroke_dasharray: "40",
                                            stroke_dashoffset: "10",
                                        }
                                    }
                                    "Logging in…"
                                }
                            },
                            AuthState::LoggedOut => rsx! {
                                button {
                                    class: "{AUTH_BTN}",
                                    onclick: move |_| {
                                        let path = use_route::<Route>();
                                        let (auth_state, auth_url) = AuthState::login(path.to_string());
                                        storage_signal.write().set_auth_state(auth_state);
                                        window().unwrap().location().set_href(&auth_url).unwrap();
                                    },
                                    "Login"
                                }
                            },
                            AuthState::LoggedIn(_) => rsx! {
                                button {
                                    class: "{AUTH_BTN}",
                                    onclick: move |_| {
                                        let path = use_route::<Route>();
                                        let (auth_state, auth_url) = storage_signal
                                            .read()
                                            .get_auth_state()
                                            .logout(&path.to_string());
                                        storage_signal.write().set_auth_state(auth_state);
                                        window().unwrap().location().set_href(&auth_url).unwrap();
                                    },
                                    img {
                                        src: PROVILE,
                                        alt: "Profile",
                                        class: "h-6 w-6 rounded-full border border-amber-200",
                                    }
                                    "Logout"
                                }
                            },
                            AuthState::Error(_) => rsx! {
                                button {
                                    class: "{AUTH_BTN} border-red-200 text-red-600 bg-red-50 hover:bg-red-100",
                                    onclick: move |_| {
                                        let path = use_route::<Route>();
                                        let (auth_state, auth_url) = AuthState::login(path.to_string());
                                        storage_signal.write().set_auth_state(auth_state);
                                        window().unwrap().location().set_href(&auth_url).unwrap();
                                    },
                                    "Retry Login"
                                }
                            },
                        }
                    }
                }
            }

            // ── Page content ──────────────────────────────────────
            main { class: "flex h-full w-full", Outlet::<Route> {} }
        }
    }
}

// ─────────────────────────────────────────────
//  App root
// ─────────────────────────────────────────────

#[component]
fn App() -> Element {
    use_context_provider(|| {
        let auth_state = AuthState::new();
        let storage = StorageManager::new(auth_state);
        match storage {
            Ok(s) => Signal::new(s),
            Err(err) => {
                console::error_1(&format!("Fatal: {}", err).into());
                panic!("Storage failed: {}", err);
            }
        }
    });
    rsx! {
        Router::<Route> {}
    }
}

// ─────────────────────────────────────────────
//  Inline error helper
// ─────────────────────────────────────────────

pub fn error(headline: &str, message: &str) -> Element {
    rsx! {
        div {
            class: "flex gap-3 p-4 rounded-xl border border-red-200 bg-red-50",
            role: "alert",
            svg {
                class: "shrink-0 w-4 h-4 mt-0.5 text-red-500",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                view_box: "0 0 20 20",
                path { d: "M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z" }
            }
            div { class: "text-sm",
                span { class: "font-semibold text-red-700", "{headline}" }
                p { class: "text-red-600 mt-0.5", "{message}" }
            }
        }
    }
}
