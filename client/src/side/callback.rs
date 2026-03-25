use dioxus::prelude::*;
use web_sys::console;

use crate::{config::AppConfig, storage::StorageManager, AuthState, Route};

// ─────────────────────────────────────────────
//  CSS: animierte Punkte hinter dem Statustext
// ─────────────────────────────────────────────

const CALLBACK_CSS: &str = r#"
@keyframes callback-dots {
    0%   { content: ""; }
    33%  { content: "."; }
    66%  { content: ".."; }
    100% { content: "..."; }
}
.callback-dot::after {
    content: "";
    animation: callback-dots 1.4s steps(1, end) infinite;
}
"#;

#[component]
pub fn Callback(code: String, state: String) -> Element {
    console::debug_1(&format!("URL Params - code: {:?}, state: {:?}", code, state).into());
    let config_resource = use_resource(move || async move { AppConfig::fetch().await });

    let config = match &*config_resource.read_unchecked() {
        None => {
            return rsx! {
                CallbackScreen { status: CallbackStatus::Loading }
            };
        }
        Some(Err(e)) => {
            console::error_1(&format!("Error while loading auth config: {}", e).into());
            return rsx! {
                CallbackScreen { status: CallbackStatus::Error }
            };
        }
        Some(Ok(config)) => config.clone(),
    };

    let mut storage_signal = use_context::<Signal<StorageManager>>();

    use_effect(move || {
        let code = code.clone();
        let state = state.clone();
        let config = config.clone();
        spawn(async move {
            let auth = AuthState::new().callback(&config, &code, &state).await;
            let result = storage_signal.write().set_auth_state(auth);
            if let Err(e) = result {
                console::error_1(&format!("Error while setting auth state: {}", e).into());
            }
            let route = Route::from_string(&state);
            navigator().push(route);
        });
    });

    rsx! {
        CallbackScreen { status: CallbackStatus::Authenticating }
    }
}

// ─────────────────────────────────────────────
//  Status enum
// ─────────────────────────────────────────────

#[derive(PartialEq, Clone)]
enum CallbackStatus {
    Loading,
    Authenticating,
    Error,
}

// ─────────────────────────────────────────────
//  Screen
//  Callback lebt in: Wrapper > main.flex.h-full.w-full > Outlet
//  → flex-1 füllt den verbleibenden vertikalen Raum nach dem Header
// ─────────────────────────────────────────────

#[component]
fn CallbackScreen(status: CallbackStatus) -> Element {
    let is_error = status == CallbackStatus::Error;

    rsx! {
        style { dangerous_inner_html: CALLBACK_CSS }

        div { class: "flex-1 flex items-center justify-center w-full px-8 py-6 space-y-8",
            div { class: "text-center",

                if is_error {
                    // ── Fehler-Icon ──────────────────────────────
                    div { class: "inline-flex items-center justify-center \
                                  rounded-full h-12 w-12 \
                                  border-4 border-red-100 bg-red-50",
                        svg {
                            class: "w-5 h-5 text-red-400",
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "2.5",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M6 18L18 6M6 6l12 12",
                            }
                        }
                    }
                    p { class: "mt-4 text-[#70513E] text-sm font-semibold tracking-wide",
                        "Authentication failed"
                    }
                    p { class: "mt-1 text-xs text-[#70513E]/60",
                        "Please close this tab and try again."
                    }
                } else {
                    // ── Spinner – gleiche Größe & Farben wie LoadingPage ──
                    div { class: "inline-block relative",
                        div { class: "animate-spin rounded-full h-12 w-12 \
                                      border-4 border-[#F1E7D7] border-t-[#D67229]" }
                        // Lock-Icon zentriert im Ring
                        div { class: "absolute inset-0 flex items-center justify-center",
                            svg {
                                class: "w-4 h-4 text-[#D67229]",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 \
                                        11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 \
                                        2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 \
                                        00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z",
                                }
                            }
                        }
                    }
                    p { class: "mt-4 text-[#70513E] text-sm font-semibold tracking-wide",
                        "Authenticating"
                        span { class: "callback-dot" }
                    }
                }
            }
        }
    }
}
