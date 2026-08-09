mod address;
pub mod calculate;
pub mod courses;
pub mod overview;
pub mod plan;
mod run_schedule;
pub mod share_team;
pub mod startend;
pub mod teams;

use dioxus::prelude::*;
use uuid::Uuid;

use crate::Route;
use crate::ui::{
    cards::{BaseCard, CardHeader},
    icons::{
        CalendarIcon, ChevronRightIcon, ClockIcon, CloseIcon, SpinnerIcon, UserGroupIcon,
    },
    tokens::FOCUS_RING,
    typography::{CaptionText, Headline2, SubText, Text},
};

// ─────────────────────────────────────────────
//  Navigation Item Helper
// ─────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct NavItem {
    label: &'static str,
    target_route: Route,
    icon: Element,
}

// ─────────────────────────────────────────────
//  Sidebar / Menu Component
// ─────────────────────────────────────────────

#[component]
pub fn Menu(cook_and_run_id: Uuid) -> Element {
    let current_route = use_route::<Route>();
    let mut is_mobile_open = use_signal(|| false);
    let mut is_collapsed = use_signal(|| false);

    let nav_items = vec![
        NavItem {
            label: "Overview",
            target_route: Route::Overview { cook_and_run_id },
            icon: rsx! {
                svg {
                    class: "w-5 h-5 shrink-0",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z",
                    }
                }
            },
        },
        NavItem {
            label: "Teams",
            target_route: Route::Teams { cook_and_run_id },
            icon: rsx! {
                UserGroupIcon { class: Some("w-5 h-5 shrink-0".to_string()) }
            },
        },
        NavItem {
            label: "Start & End",
            target_route: Route::StartEnd { cook_and_run_id },
            icon: rsx! {
                CalendarIcon { class: Some("w-5 h-5 shrink-0".to_string()) }
            },
        },
        NavItem {
            label: "Courses",
            target_route: Route::Courses { cook_and_run_id },
            icon: rsx! {
                ClockIcon { class: Some("w-5 h-5 shrink-0".to_string()) }
            },
        },
        NavItem {
            label: "Calculation",
            target_route: Route::Calculate { cook_and_run_id },
            icon: rsx! {
                svg {
                    class: "w-5 h-5 shrink-0",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M15.75 15.75V18m-7.5-6.75h.008v.008H8.25v-.008zm0 2.25h.008v.008H8.25V13.5zm0 2.25h.008v.008H8.25v-.008zm0 2.25h.008v.008H8.25V18zm2.498-6.75h.007v.008h-.007v-.008zm0 2.25h.007v.008h-.007V13.5zm0 2.25h.007v.008h-.007v-.008zm0 2.25h.007v.008h-.007V18zm2.504-6.75h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V13.5zm0 2.25h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V18zm2.498-6.75h.008v.008h-.008v-.008zm0 2.25h.008v.008h-.008V13.5zM8.25 6h7.5a1.5 1.5 0 011.5 1.5v1.5a1.5 1.5 0 01-1.5 1.5h-7.5A1.5 1.5 0 016 9V7.5A1.5 1.5 0 018.25 6zM6 20.25h12A2.25 2.25 0 0020.25 18V6A2.25 2.25 0 0018 3.75H6A2.25 2.25 0 003.75 6v12A2.25 2.25 0 006 20.25z",
                    }
                }
            },
        },
    ];

    let is_collapsed_val = *is_collapsed.read();
    let is_mobile_open_val = *is_mobile_open.read();

    rsx! {
        // Root Wrapper mit strikter Überlauf-Sperre (overflow-x-hidden, max-w-full)
        div { class: "min-h-screen w-full max-w-full overflow-x-hidden flex flex-col md:flex-row bg-amber-50/20 text-zinc-800",

            // ── Mobile Top Header (< md) ─────────────────────────────
            header { class: "md:hidden flex items-center justify-between px-4 py-3 bg-white border-b border-amber-100 shadow-2xs sticky top-0 z-40 w-full shrink-0",
                div { class: "flex items-center gap-2.5",
                    div { class: "w-2 h-5 rounded-full bg-amber-500" }
                    Headline2 { headline: "Cook & Run".to_string() }
                }
                button {
                    r#type: "button",
                    class: "p-2 text-zinc-600 hover:text-amber-800 hover:bg-amber-100/50 rounded-xl transition-all {FOCUS_RING}",
                    onclick: move |_| is_mobile_open.set(!is_mobile_open_val),
                    aria_label: "Toggle Navigation Menu",
                    if is_mobile_open_val {
                        CloseIcon { class: Some("w-6 h-6".to_string()) }
                    } else {
                        svg {
                            class: "w-6 h-6",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                            }
                        }
                    }
                }
            }

            // ── Mobile Drawer Overlay ───────────────────────────────
            if is_mobile_open_val {
                div {
                    class: "md:hidden fixed inset-0 top-[57px] z-30 bg-zinc-900/30 backdrop-blur-xs animate-in fade-in duration-200",
                    onclick: move |_| is_mobile_open.set(false),
                }
                div { class: "md:hidden fixed top-[57px] left-0 right-0 z-30 bg-white border-b border-amber-100 shadow-xl p-4 space-y-1.5 animate-in slide-in-from-top-2 duration-200 max-h-[calc(100vh-57px)] overflow-y-auto",
                    for item in nav_items.iter() {
                        {
                            let item_clone = item.clone();
                            let is_active = item_clone.target_route == current_route;
                            let active_cls = "bg-amber-500 text-white shadow-xs font-semibold";
                            let inactive_cls = "text-zinc-700 hover:bg-amber-100/50 hover:text-amber-900 font-medium";
                            rsx! {
                                button {
                                    key: "{item_clone.label}",
                                    r#type: "button",
                                    class: format_args!(
                                        "w-full flex items-center gap-3 px-4 py-3 rounded-xl text-sm transition-all {} {}",
                                        FOCUS_RING,
                                        if is_active { active_cls } else { inactive_cls },
                                    ),
                                    onclick: move |_| {
                                        is_mobile_open.set(false);
                                        use_navigator().push(item_clone.target_route.clone());
                                    },
                                    {item_clone.icon.clone()}
                                    span { "{item_clone.label}" }
                                }
                            }
                        }
                    }
                }
            }

            // ── Desktop Sidebar (>= md) ──────────────────────────────
            nav {
                class: format_args!(
                    "hidden md:flex flex-col justify-between shrink-0 h-screen sticky top-0 bg-white/90 backdrop-blur-md border-r border-amber-100/80 shadow-2xs transition-all duration-300 p-4 z-20 {}",
                    if is_collapsed_val { "w-20" } else { "w-64" },
                ),

                // Top Header / Logo Section
                div { class: "space-y-6",
                    div { class: "flex items-center justify-between px-2 py-1",
                        if !is_collapsed_val {
                            div { class: "flex items-center gap-2.5 min-w-0",
                                div { class: "w-2 h-5 rounded-full bg-amber-500 shrink-0" }
                                Headline2 { headline: "Cook & Run".to_string() }
                            }
                        }
                        button {
                            r#type: "button",
                            class: "p-1.5 text-zinc-400 hover:text-amber-800 hover:bg-amber-100/50 rounded-xl transition-all mx-auto {FOCUS_RING}",
                            onclick: move |_| is_collapsed.set(!is_collapsed_val),
                            title: if is_collapsed_val { "Expand menu" } else { "Collapse menu" },
                            ChevronRightIcon {
                                class: Some(
                                    format!(
                                        "w-5 h-5 transition-transform duration-200 {}",
                                        if is_collapsed_val { "" } else { "rotate-180" },
                                    ),
                                ),
                            }
                        }
                    }

                    // Navigation Links
                    ul { class: "space-y-1.5",
                        for item in nav_items.iter() {
                            {
                                let item_clone = item.clone();
                                let is_active = item_clone.target_route == current_route;
                                let active_cls = "bg-amber-500 text-white shadow-xs font-semibold";
                                let inactive_cls = "text-zinc-700 hover:bg-amber-100/60 hover:text-amber-900 font-medium";
                                rsx! {
                                    li { key: "{item_clone.label}",
                                        button {
                                            r#type: "button",
                                            class: format_args!(
                                                "w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-sm transition-all cursor-pointer {} {}",
                                                FOCUS_RING,
                                                if is_active { active_cls } else { inactive_cls },
                                            ),
                                            title: if is_collapsed_val { item_clone.label } else { "" },
                                            onclick: move |_| {
                                                use_navigator().push(item_clone.target_route.clone());
                                            },
                                            div { class: "shrink-0", {item_clone.icon.clone()} }
                                            if !is_collapsed_val {
                                                span { class: "truncate", "{item_clone.label}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            
            }

            // ── Main Page Content Outlet ────────────────────────────
            // Hier erfolgt nun die ZENTRIERUNG und maximale Breitenbegrenzung für alle Seiten
            main { class: "flex-1 min-w-0 w-full overflow-y-auto overflow-x-hidden p-3 sm:p-6 lg:p-8 flex flex-col items-center",
                div { class: "w-full max-w-5xl flex flex-col items-center", Outlet::<Route> {} }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Loading Page
// ─────────────────────────────────────────────

#[component]
pub(crate) fn LoadingPage() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[60vh] w-full p-6 space-y-3",
            SpinnerIcon { class: Some("w-10 h-10 text-amber-500".to_string()) }
            SubText {
                text: "Loading project data…".to_string(),
                class: Some("font-medium text-zinc-600".to_string()),
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Error Page
// ─────────────────────────────────────────────

#[component]
pub(crate) fn ErrorPage(error_text: String, error_details: String) -> Element {
    rsx! {
        div { class: "flex items-center justify-center min-h-[60vh] w-full p-6",
            div { class: "w-full max-w-md",
                BaseCard {
                    CardHeader {
                        title: "Something went wrong".to_string(),
                        class: "bg-red-50/80 border-red-100 text-red-900".to_string(),
                    }
                    div { class: "p-6 space-y-4",
                        Text {
                            text: error_text,
                            class: Some("text-zinc-700 leading-relaxed".to_string()),
                        }
                        details { class: "group",
                            summary {
                                SubText {
                                    text: "Technical Details".to_string(),
                                    class: Some(
                                        "font-semibold tracking-wider uppercase text-zinc-400 hover:text-zinc-600 transition-colors cursor-pointer select-none"
                                            .to_string(),
                                    ),
                                }
                            }
                            p { class: "mt-2 text-xs text-zinc-500 whitespace-pre-wrap bg-zinc-50 rounded-xl border border-zinc-200/60 p-3 font-mono leading-relaxed",
                                "{error_details}"
                            }
                        }
                    }
                }
            }
        }
    }
}