mod address;
pub mod calculate;
pub mod courses;
pub mod overview;
pub mod plan;
mod run_schedule;
pub mod startend;
pub mod teams;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::Route;

// ─────────────────────────────────────────────
//  Sidebar / Menu
// ─────────────────────────────────────────────

#[component]
pub fn Menu(cook_and_run_id: Uuid) -> Element {
    let current_route = use_route::<Route>();

    rsx!(
        nav { class: "w-64 h-full bg-[#F8EFE1] px-4 py-6 shadow-md rounded-r-2xl flex flex-col",

            // Eyebrow label
            h2 { class: "text-lg font-semibold text-[#70513E] px-2 mb-5 tracking-wide",
                "Menu"
            }

            ul { class: "space-y-1",
                SidebarButton {
                    label: "Overview",
                    target_route: Route::Overview { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Teams",
                    target_route: Route::Teams { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Start & End",
                    target_route: Route::StartEnd { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Courses",
                    target_route: Route::Courses { cook_and_run_id },
                    current_route: current_route.clone(),
                }
                SidebarButton {
                    label: "Calculation",
                    target_route: Route::Calculate { cook_and_run_id },
                    current_route: current_route.clone(),
                }
            }
        }
        Outlet::<Route> {}
    )
}

#[component]
fn SidebarButton(label: String, target_route: Route, current_route: Route) -> Element {
    let base = "block text-left w-full px-4 py-2 rounded-xl text-sm font-medium \
                transition-colors duration-150";
    let active = "bg-[#D67229] text-white shadow-sm";
    let inactive = "text-[#70513E] hover:text-[#C66741] hover:bg-[#F1E7D7]";
    let is_active = target_route.eq(&current_route);
    rsx! {
        li {
            button {
                class: format_args!("{} {}", base, if is_active { active } else { inactive }),
                onclick: move |_| {
                    let t = target_route.clone();
                    use_navigator().push(t);
                },
                "{label}"
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Loading page
// ─────────────────────────────────────────────

#[component]
pub(crate) fn LoadingPage() -> Element {
    rsx! {
        div { class: "flex items-center justify-center h-screen w-full",
            div { class: "text-center",
                div { class: "inline-block",
                    div { class: "animate-spin rounded-full h-12 w-12 \
                                  border-4 border-[#F1E7D7] border-t-[#D67229]" }
                }
                p { class: "mt-4 text-[#70513E] text-sm font-semibold tracking-wide", "Loading…" }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Error page
// ─────────────────────────────────────────────

#[component]
pub(crate) fn ErrorPage(error_text: String, error_details: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-center h-screen w-full px-6",
            div { class: "w-full max-w-md",

                // Error card
                div { class: "bg-white rounded-2xl border border-red-100 shadow-sm overflow-hidden",

                    // Red header stripe
                    div { class: "px-5 py-3.5 bg-red-50/70 border-b border-red-100 \
                                  flex items-center gap-2.5",
                        div { class: "w-1.5 h-5 rounded-full bg-red-400/70" }
                        span { class: "text-sm font-semibold text-red-700", "Something went wrong" }
                    }

                    // Body
                    div { class: "px-5 py-5 space-y-3",
                        p { class: "text-sm text-zinc-700", "{error_text}" }

                        details { class: "group",
                            summary { class: "text-[11px] font-semibold tracking-[0.1em] uppercase \
                                             text-zinc-400 cursor-pointer \
                                             hover:text-zinc-600 transition-colors select-none",
                                "Details"
                            }
                            p { class: "mt-2 text-xs text-zinc-500 whitespace-pre-wrap \
                                        bg-zinc-50 rounded-xl border border-zinc-100 \
                                        px-3 py-2 leading-relaxed",
                                "{error_details}"
                            }
                        }
                    }
                }
            }
        }
    }
}
