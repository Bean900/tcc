use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::{console, wasm_bindgen::JsCast, HtmlInputElement};

use crate::{
    async_action,
    side::{
        AsyncAction, CloseButton, ConfirmButton, ErrorSVG, Headline1, Input, InputError,
        SecondaryButton,
    },
    storage::{CookAndRunCreate, CookAndRunData, StorageManager},
};

// ─────────────────────────────────────────────
//  Shared tokens
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

const NATIVE_INPUT: &str = "w-full px-3 py-2 rounded-xl border border-amber-200 bg-amber-50/40 \
     text-sm text-zinc-800 placeholder-zinc-400 \
     focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 \
     transition-colors duration-150";

// ─────────────────────────────────────────────
//  Dashboard
// ─────────────────────────────────────────────

#[component]
pub fn Dashboard() -> Element {
    console::debug_1(&"Rendering Dashboard...".into());
    let storage = use_context::<Signal<StorageManager>>();
    let cook_and_run_list = use_resource(move || async move {
        console::debug_1(&"Loading cook and run list...".into());
        let storage = storage.read();
        match storage.select_cook_and_run_meta_list().await {
            Ok(list) => Ok(list),
            Err(err) => {
                console::error_1(&format!("Failed to load cook and run list: {}", err).into());
                Err("Failed to load data!".to_string())
            }
        }
    });

    let mut create_project_signal = use_signal(|| false);

    rsx! {
        div { class: "px-8 py-6 space-y-8",

            // ── Page header ───────────────────────────────────────
            Headline1 { headline: "Projects" }

            // ── Project grid ──────────────────────────────────────
            div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-5",

                match &*cook_and_run_list.read_unchecked() {
                    Some(Err(err)) => rsx! {
                        div { class: "col-span-full flex items-center gap-3 \
                                      rounded-2xl border border-red-200 bg-red-50 px-5 py-4",
                            ErrorSVG {}
                            span { class: "text-sm text-red-700", "{err}" }
                        }
                    },
                    Some(Ok(list)) => rsx! {
                        {
                            list.iter().map(|cook_and_run| {
                                rsx! {
                                    DashboardCard {
                                        id: cook_and_run.id,
                                        name: cook_and_run.name.clone(),
                                        created: cook_and_run.created.format("%Y-%m-%d %H:%M").to_string(),
                                        updated: cook_and_run.edited.format("%Y-%m-%d %H:%M").to_string(),
                                        uploaded: cook_and_run.is_in_cloud,
                                    }
                                }
                            })
                        }
                    },
                    None => rsx! {
                        LoadingCard {}
                        LoadingCard {}
                        LoadingCard {}
                    },
                }

                // ── Add project button ────────────────────────────
                a {
                    class: "flex flex-col items-center justify-center gap-3 h-36 \
                            rounded-2xl border-2 border-dashed border-amber-200 \
                            bg-amber-50/30 \
                            text-amber-400 hover:text-amber-600 \
                            hover:border-amber-400 hover:bg-amber-50/60 \
                            transition-all duration-200 cursor-pointer group",
                    onclick: move |_| { create_project_signal.set(true); },
                    div { class: "w-9 h-9 rounded-full border-2 border-current \
                                  flex items-center justify-center \
                                  text-xl font-bold leading-none \
                                  group-hover:scale-110 transition-transform duration-150",
                        "+"
                    }
                    span { class: "text-sm font-semibold tracking-wide", "New project" }
                }
            }
        }

        // ── Create project dialog ─────────────────────────────────
        if *create_project_signal.read() {
            CreateProjectDialog { create_project_signal: create_project_signal.clone() }
        }
    }
}

// ─────────────────────────────────────────────
//  Dashboard card
// ─────────────────────────────────────────────

#[derive(PartialEq, Props, Clone)]
struct DashboardCardProps {
    id: Uuid,
    name: String,
    created: String,
    updated: String,
    uploaded: bool,
}

#[component]
fn DashboardCard(props: DashboardCardProps) -> Element {
    rsx! {
        a {
            href: format!("/cook-and-run/{}/overview", props.id),
            class: "relative flex flex-col bg-white rounded-2xl border border-amber-100 \
                    shadow-sm hover:shadow-md transition-all duration-150 cursor-pointer \
                    overflow-hidden group",

            // Amber header stripe
            div { class: "px-4 py-2.5 bg-amber-50/70 border-b border-amber-100 \
                          flex items-center justify-between gap-2",
                div { class: "flex items-center gap-2 min-w-0",
                    div { class: "w-1.5 h-4 rounded-full bg-amber-400/70 shrink-0" }
                    span { class: "text-sm font-semibold text-zinc-800 truncate",
                        "{props.name}"
                    }
                }
                // Cloud indicator
                if props.uploaded {
                    svg {
                        class: "w-4 h-4 shrink-0 text-amber-500",
                        fill: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M16.88 9.94a5 5 0 00-9.72-1.47A4 4 0 006 17h9a4 4 0 001.88-7.06z" }
                    }
                } else {
                    svg {
                        class: "w-4 h-4 shrink-0 text-zinc-300",
                        fill: "currentColor",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M16.88 9.94a5 5 0 00-9.72-1.47A4 4 0 006 17h9a4 4 0 001.88-7.06z" }
                    }
                }
            }

            // Card body
            div { class: "px-4 py-3 space-y-1.5",

                // Created
                div { class: "flex items-center gap-1.5 text-xs text-zinc-500",
                    svg {
                        class: "w-3.5 h-3.5 shrink-0 text-zinc-400",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zM2 9v7a2 2 0 002 2h12a2 2 0 002-2V9H2z" }
                    }
                    span { "{props.created}" }
                }

                // Updated
                div { class: "flex items-center gap-1.5 text-xs text-zinc-400",
                    svg {
                        class: "w-3.5 h-3.5 shrink-0 text-zinc-300",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M17.414 2.586a2 2 0 010 2.828l-8.586 8.586a2 2 0 01-.879.515l-4 1a1 1 0 01-1.213-1.213l1-4a2 2 0 01.515-.879l8.586-8.586a2 2 0 012.828 0zM15 5l-1-1L6 12l-.5 2 .5.5 2-.5L15 5z" }
                    }
                    span { "{props.updated}" }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Loading skeleton card
// ─────────────────────────────────────────────

#[component]
fn LoadingCard() -> Element {
    rsx! {
        div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden",

            // Skeleton header stripe
            div { class: "px-4 py-2.5 bg-amber-50/70 border-b border-amber-100",
                div { class: "h-4 w-32 bg-amber-100 rounded-full animate-pulse" }
            }

            // Skeleton body
            div { class: "px-4 py-3 space-y-2.5",
                div { class: "flex items-center gap-1.5",
                    div { class: "w-3.5 h-3.5 rounded-full bg-zinc-200 animate-pulse shrink-0" }
                    div { class: "h-3 w-28 bg-zinc-200 rounded-full animate-pulse" }
                }
                div { class: "flex items-center gap-1.5",
                    div { class: "w-3.5 h-3.5 rounded-full bg-zinc-100 animate-pulse shrink-0" }
                    div { class: "h-3 w-24 bg-zinc-100 rounded-full animate-pulse" }
                }
            }
        }
        span { class: "sr-only", "Loading…" }
    }
}

// ─────────────────────────────────────────────
//  Create project dialog
// ─────────────────────────────────────────────

#[component]
fn CreateProjectDialog(create_project_signal: Signal<bool>) -> Element {
    let mut project_name_signal = use_signal(|| "".to_string());
    let mut error_name_signal = use_signal(|| "".to_string());
    let mut error_signal = use_signal(|| "".to_string());

    rsx! {
        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen \
                      justify-center items-center bg-black/20 z-50",
            div { class: "relative bg-white rounded-2xl border border-amber-100 \
                          shadow-xl w-80 overflow-hidden",

                // Dialog header
                div { class: "px-5 py-4 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5",
                    div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                    span { class: "text-base font-semibold text-zinc-800", "New Project" }
                }

                // Close button
                CloseButton {
                    onclick: move |_| { create_project_signal.set(false); },
                }

                // Form body
                div { class: "px-5 py-5 space-y-4",

                    div {
                        label { class: "{LBL}", "Project Name" }
                        Input {
                            place_holer: Some("e.g. Summer Cook & Run 2025".to_string()),
                            value: project_name_signal.clone(),
                            is_error: !error_name_signal.read().is_empty(),
                            oninput: move |e: Event<FormData>| {
                                let value = e.value();
                                project_name_signal.set(value.clone());
                                if value.trim().is_empty() {
                                    error_name_signal.set("Project name cannot be empty!".to_string());
                                } else {
                                    error_name_signal.set("".to_string());
                                }
                            },
                        }
                        InputError { error: error_name_signal.read() }
                    }

                    // Hidden file input
                    input {
                        id: "project_upload",
                        r#type: "file",
                        accept: ".tcc",
                        hidden: true,
                        multiple: false,
                        onchange: move |evt| {
                            async move {
                                for file_name in &evt.files() {
                                    let file_content = match file_name.read_string().await {
                                        Ok(c) => c,
                                        Err(e) => {
                                            console::error_1(
                                                &format!("Error reading project file: {}", e).into(),
                                            );
                                            error_signal.set("Error reading project file!".to_string());
                                            continue;
                                        }
                                    };
                                    let mut storage = use_context::<Signal<StorageManager>>();
                                    let mut storage = storage.write();
                                    let cook_and_run = match CookAndRunData::from_json(&file_content) {
                                        Ok(c) => c,
                                        Err(e) => {
                                            console::error_1(
                                                &format!("Error parsing project file: {}", e).into(),
                                            );
                                            error_signal.set("Error parsing project file!".to_string());
                                            continue;
                                        }
                                    };
                                    let result = storage.create_from_file(cook_and_run).await;
                                    if let Err(e) = result {
                                        console::error_1(
                                            &format!("Error creating project from file: {}", e).into(),
                                        );
                                        error_signal.set("Error creating project from file!".to_string());
                                    } else {
                                        create_project_signal.set(false);
                                        todo!();
                                    }
                                }
                            }
                        },
                    }

                    InputError { error: error_signal.read() }

                    // Action buttons
                    div { class: "flex gap-3 pt-1",
                        SecondaryButton {
                            text: "Upload".to_string(),
                            action: async_action!(
                                {
                                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                        if let Some(el) = doc.get_element_by_id("project_upload") {
                                            if let Ok(input) = el.dyn_into::<HtmlInputElement>() {
                                                input.click();
                                            }
                                        }
                                    }
                                }
                            ),
                        }
                        ConfirmButton {
                            text: "Create".to_string(),
                            error_signal: error_signal.clone(),
                            action: async_action!(
                                {
                                    if project_name_signal.read().trim().is_empty() {
                                        error_name_signal.set("Project name cannot be empty!".to_string());
                                        return;
                                    }
                                    let project_id = Uuid::new_v4();
                                    let mut storage = use_context::<Signal<StorageManager>>();
                                    let mut storage = storage.write();
                                    let cook_and_run = CookAndRunCreate {
                                        name: project_name_signal.read().to_string(),
                                    };
                                    let result = storage
                                        .create_cook_and_run(project_id, &cook_and_run)
                                        .await;
                                    if let Err(e) = result {
                                        console::error_1(
                                            &format!("Error creating project: {}", e).into(),
                                        );
                                        error_signal.set("Creating project failed!".to_string());
                                        return;
                                    }
                                    create_project_signal.set(false);
                                }
                            ),
                        }
                    }
                }
            }
        }
    }
}
