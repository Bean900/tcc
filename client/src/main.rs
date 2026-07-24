mod address_connector;
mod calculator;
pub mod config;
pub mod keycloak;
mod side;
mod storage;

use crate::config::Config;
use crate::config::LegalConfig;
use crate::footer::Footer;
use dioxus::prelude::*;
use side::Calculate;
use side::Callback;
use side::CookieBanner;
use side::CookieSettings;
use side::Courses;
use side::Dashboard;
use side::Impressum;
use side::Overview;
use side::Plan;
use side::Privacy;
use side::ShareRegisterPage;
use side::StartEnd;
use side::Teams;

use uuid::Uuid;

mod footer;
mod state;

use web_sys::console;
use web_sys::window;

pub use crate::keycloak::AuthState;
use crate::side::Menu;
use crate::state::ConsentState;
use crate::storage::{ StorageManager};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const PROFILE: Asset = asset!("/assets/profile.png");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");
const LOGO: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus::launch(App);
}

// ─────────────────────────────────────────────
//  Toast-Modell & Hilfsfunktion
// ─────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub struct ToastMessage {
    pub id: Uuid,
    pub headline: String,
    pub message: String,
}

pub fn trigger_error_toast(mut toasts: Signal<Vec<ToastMessage>>, headline: &str, message: &str) {
    let id = Uuid::new_v4();
    toasts.write().push(ToastMessage {
        id,
        headline: headline.to_string(),
        message: message.to_string(),
    });

    spawn(async move {
        gloo_timers::future::TimeoutFuture::new(15000).await;
        toasts.write().retain(|t| t.id != id);
    });
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    // ── Haupt-App (mit Header + Auth-Button + Footer) ──
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/callback?:code&:state")]
        Callback { code: String, state: String },
        #[nest("/cook-and-run")]
            #[route("/")]
            Dashboard {},
            #[route("/:cook_and_run_id/share")]
            ShareRegisterPage { cook_and_run_id: Uuid },
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
            #[route("/impressum")]
            Impressum {},
            #[route("/datenschutz")]
            Privacy {},
            #[route("/cookies")]
            CookieSettings {},
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

impl Route {
    fn to_string(&self) -> String {
        match self {
            Route::Home {} => "-".to_string(),
            Route::Callback {
                code: _code,
                state: _state,
            } => "-".to_string(),
            Route::Dashboard {} => "cook-and-run".to_string(),
            Route::Overview { cook_and_run_id } => format!("overview.{}", cook_and_run_id),
            Route::Teams { cook_and_run_id } => format!("teams.{}", cook_and_run_id),
            Route::StartEnd { cook_and_run_id } => format!("startend.{}", cook_and_run_id),
            Route::Courses { cook_and_run_id } => format!("courses.{}", cook_and_run_id),
            Route::Calculate { cook_and_run_id } => format!("calculate.{}", cook_and_run_id),
            Route::Plan {
                cook_and_run_id,
                team_id,
            } => format!("plan.{}.{}", cook_and_run_id, team_id),
            _ => "cook-and-run".to_string(),
        }
    }

    fn from_string(s: &str) -> Self {
        let parts: Vec<&str> = s.split('.').collect();
        match parts[0] {
            "-" => Route::Home {},
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
            "plan" => {
                if parts.len() == 3 {
                    if let (Ok(cook_and_run_id), Ok(team_id)) =
                        (Uuid::parse_str(parts[1]), Uuid::parse_str(parts[2]))
                    {
                        return Route::Plan {
                            cook_and_run_id,
                            team_id,
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
        div { class: "w-full flex flex-col items-center justify-center h-screen px-6",
            div { class: "text-center mb-10",
                p { class: "text-[11px] font-semibold tracking-[0.15em] uppercase text-amber-600 mb-2",
                    "Cook & Run"
                }
                h1 { class: "text-4xl font-bold text-zinc-900 mb-3", "Traveling Cook Calculator" }
                p { class: "text-base text-zinc-500 max-w-sm mx-auto leading-relaxed",
                    "Plan your cooking and running events with ease."
                }
            }
            div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm px-8 py-6 flex flex-col items-center gap-4",
                Link {
                    to: Route::Dashboard {},
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
                        Link {
                            to: Route::Dashboard {},
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
//  Toast Container
// ─────────────────────────────────────────────

#[component]
pub fn ToastContainer() -> Element {
    let mut toasts = use_context::<Signal<Vec<ToastMessage>>>();

    rsx! {
        div { class: "fixed bottom-6 right-6 z-[100] flex flex-col gap-3 max-w-sm w-full pointer-events-none",
            for toast in toasts.read().iter().cloned() {
                div {
                    key: "{toast.id}",
                    class: "pointer-events-auto relative overflow-hidden flex gap-3.5 p-4 bg-white rounded-xl \
                            shadow-[0_8px_30px_rgb(0,0,0,0.12)] border border-zinc-100 border-l-4 border-l-red-500 \
                            transition-all duration-300 transform translate-y-0 animate-fade-in-up",
                    role: "alert",

                    div { class: "shrink-0 mt-0.5",
                        svg {
                            class: "w-5 h-5 text-red-500",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
                            }
                        }
                    }

                    div { class: "flex-1 pr-2",
                        h3 { class: "text-sm font-semibold text-zinc-900", "{toast.headline}" }
                        p { class: "text-sm text-zinc-500 mt-1 leading-relaxed whitespace-pre-line",
                            "{toast.message}"
                        }
                    }

                    button {
                        class: "absolute top-4 right-4 text-zinc-400 hover:text-zinc-700 transition-colors duration-200",
                        onclick: move |_| {
                            toasts.write().retain(|t| t.id != toast.id);
                        },
                        svg {
                            class: "w-4 h-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M6 18L18 6M6 6l12 12",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Wrapper() -> Element {
    let current_route = use_route::<Route>();
    let route_state = current_route.to_string();

    const AUTH_BTN: &str = "flex items-center gap-2 px-3 py-1.5 rounded-xl text-sm font-medium \
         text-zinc-600 bg-amber-50 border border-amber-200 \
         hover:bg-amber-100 hover:border-amber-300 \
         transition-colors duration-150 cursor-pointer";

    let spinner_svg = rsx! {
        svg {
            class: "w-3.5 h-3.5 animate-spin text-amber-400",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            circle {
                cx: "12",
                cy: "12",
                r: "10",
                stroke: "currentColor",
                stroke_width: "3",
                stroke_dasharray: "40",
                stroke_dashoffset: "10",
            }
        }
    };

    let config_signal = use_context::<Signal<Option<Config>>>();
    if let None = config_signal.read().clone() {
        console::error_1(&format!("Error: Config context is None").into());
        return rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }

            div { class: "min-h-screen w-full flex flex-col bg-[#F8EFE1]",
                header { class: "sticky top-0 z-50 bg-[#FDFAF6] border-b border-amber-200/60 shadow-sm",
                    div { class: "max-w-7xl mx-auto px-6 py-3 flex justify-between items-center w-full",
                        Link {
                            to: Route::Dashboard {},
                            class: "flex items-center gap-3",
                            img {
                                src: LOGO,
                                alt: "Cook & Run",
                                class: "h-8 w-auto",
                            }
                        }
                    }
                }

                main { class: "flex-1 flex flex-col items-center justify-center p-6 text-center",
                    svg {
                        class: "w-16 h-16 text-red-500 mb-4",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                        }
                    }

                    h1 { class: "text-2xl font-bold text-gray-800 mb-2",
                        "Failed to Load Configuration"
                    }
                    p { class: "text-gray-600 max-w-md mb-6",
                        "We encountered an unexpected error while trying to load the application settings. Please try again later or contact support."
                    }
                }
            }
        };
    }

    let mut auth_signal = use_context::<Signal<Option<AuthState>>>();
    let login = match auth_signal.read().clone() {
        None => rsx!(
            button {
                class: "{AUTH_BTN} opacity-60 cursor-not-allowed",
                disabled: true,
                {spinner_svg}
                "Loading..."
            }
        ),
        Some(loading @ AuthState::Loading(_, _, _)) => rsx! {
            button {
                class: "{AUTH_BTN} opacity-60 cursor-not-allowed",
                disabled: false,
                onclick: move |_| {
                    let value = loading.clone();
                    let route_state = route_state.clone();
                    spawn(async move {
                        let (auth, auth_url) = value.login(route_state).await;
                        auth_signal.write().replace(auth);

                        if let (Some(win), Some(url)) = (window(), auth_url) {
                            let _ = win.location().set_href(&url);
                        }
                    });
                },
                {spinner_svg}
                "Loading..."
            }
        },
        Some(logged_in @ AuthState::LoggedIn(_, _, _)) => rsx! {
            button {
                class: "{AUTH_BTN}",
                onclick: move |_| {
                    let value = logged_in.clone();
                    spawn(async move {
                        let (auth, logout_url) = value.logout().await;
                        auth_signal.write().replace(auth);

                        if let (Some(win), Some(url)) = (window(), logout_url) {
                            let _ = win.location().set_href(&url);
                        }
                    });
                },
                img {
                    src: PROFILE,
                    alt: "Profile",
                    class: "h-6 w-6 rounded-full border border-amber-200",
                }
                "Logout"
            }
        },
        Some(logged_out @ AuthState::LoggedOut(_, _)) => rsx! {
            button {
                class: "{AUTH_BTN}",
                onclick: move |_| {
                    let value = logged_out.clone();
                    let route_state = route_state.clone();
                    spawn(async move {
                        let (auth, auth_url) = value.login(route_state).await;
                        auth_signal.write().replace(auth);

                        if let (Some(win), Some(url)) = (window(), auth_url) {
                            let _ = win.location().set_href(&url);
                        }
                    });
                },
                "Login"
            }
        },
        Some(error @ AuthState::Error(_, _, _)) => rsx! {
            button {
                class: "{AUTH_BTN}",
                onclick: move |_| {
                    let value = error.clone();
                    let route_state = route_state.clone();
                    spawn(async move {
                        let (auth, auth_url) = value.login(route_state).await;
                        auth_signal.write().replace(auth);

                        if let (Some(win), Some(url)) = (window(), auth_url) {
                            let _ = win.location().set_href(&url);
                        }
                    });
                },
                "Login"
            }
        },
        Some(AuthState::NotAvailable()) => rsx! {
            button {
                class: "{AUTH_BTN} opacity-60 cursor-not-allowed",
                disabled: true,
                svg {
                    class: "w-3.5 h-3.5 text-red-400",
                    view_box: "0 0 24 24",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
                    path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
                    line {
                        x1: "2",
                        y1: "2",
                        x2: "22",
                        y2: "22",
                    }
                }
                "Not available"
            }
        },
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        ToastContainer {}

        CookieBanner {}

        div { class: "min-h-screen w-full flex flex-col bg-[#F8EFE1]",
            header { class: "sticky top-0 z-50 bg-[#FDFAF6] border-b border-amber-200/60 shadow-sm",
                div { class: "max-w-7xl mx-auto px-6 py-3 flex justify-between items-center w-full",
                    Link {
                        to: Route::Dashboard {},
                        class: "flex items-center gap-3",
                        img {
                            src: LOGO,
                            alt: "Cook & Run",
                            class: "h-8 w-auto",
                        }
                    }
                    div { class: "flex items-center gap-3", {login} }
                }
            }

            main { class: "flex flex-1 w-full", Outlet::<Route> {} }

            Footer {}
        }
    }
}

#[component]
fn App() -> Element {
    let storage = StorageManager::new().unwrap_or_else(|err| {
        console::error_1(&format!("Fatal: {}", err).into());
        panic!("Storage failed: {}", err);
    });

    let mut storage_signal = use_signal(|| storage);
    use_context_provider(|| storage_signal);

    let toasts = use_signal(Vec::<ToastMessage>::new);
    use_context_provider(|| toasts);

    let config_resource = use_resource(move || async move { Config::fetch().await });

    let mut config_signal = use_signal(|| None as Option<Config>);
    use_context_provider(|| config_signal);

    let consent_state = use_signal(|| ConsentState::load("1.0"));
    use_context_provider(|| consent_state);

    let mut auth_signal = use_signal(|| None as Option<AuthState>);
    use_context_provider(|| auth_signal);

    use_effect(move || {
        let cfg = config_resource.read_unchecked().clone();
        match cfg {
            None => {
                console::error_1(&format!("Error while loading auth config: None found!").into());
            }
            Some(Err(e)) => {
                console::error_1(&format!("Error while loading auth config: {}", e).into());
            }
            Some(Ok(config)) => {
                config_signal.set(Some(config.clone()));
                spawn(async move {
                    let auth_state = AuthState::new(config.auth).await;
                    auth_signal.write().replace(auth_state);
                });
            }
        }
    });

    use_effect(move || {
        // Diese Signale SOLLEN den Effekt auslösen:
        let auth = auth_signal.read().clone();
        let config = config_signal.read().clone();

        match (auth, config) {
            (Some(auth_state), Some(config)) => {
                spawn(async move {
                    console::debug_1(&format!("Loading cloud storage with auth state").into());
                    
                    // WICHTIG: .peek() statt .read(), um keine Reaktivitäts-Schleife zu bauen!
                    let mut storage = storage_signal.peek().clone();

                    if let Err(e) = storage
                        .load_cloud(auth_state.clone(), config.auth.audience)
                        .await
                    {
                        console::error_1(&format!("Error loading cloud storage: {}", e).into());
                    } else {
                        // Aktualisiert nur Komponenten, die auf das Signal hören, 
                        // triggert aber DIESEN Effekt wegen .peek() nicht mehr neu.
                        storage_signal.set(storage);
                    }
                });
            }
            _ => {
                console::error_1(&format!("Auth state or config is None").into());
                
                let mut storage = storage_signal.peek().clone();
                storage.disconnect_cloud();
                storage_signal.set(storage);
            }
        }
    });

    rsx! {
        Router::<Route> {}
    }
}
