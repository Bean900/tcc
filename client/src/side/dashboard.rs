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
    trigger_error_toast, Route, ToastMessage,
};

// ─────────────────────────────────────────────
//  Shared style tokens (Dioxus 0.6 & Tailwind)
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-wider uppercase text-amber-800/80 mb-1.5";

const NATIVE_INPUT: &str =
    "w-full h-10 px-3.5 py-2 rounded-xl border border-amber-200/90 bg-amber-50/30 text-sm text-zinc-800 placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-amber-400/50 focus:border-amber-400 focus:bg-white transition-all duration-150";

// Card shell with unified minimum height for grid symmetry
const CARD: &str =
    "bg-white rounded-2xl border border-amber-100/80 shadow-xs hover:border-amber-300 hover:shadow-md transition-all duration-200 overflow-hidden flex flex-col justify-between h-full min-h-[148px]";
const CARD_INTERACTIVE: &str =
    "relative hover:-translate-y-0.5 transition-all duration-200 cursor-pointer group";

const CARD_HEADER: &str =
    "px-4 py-3 bg-amber-50/60 border-b border-amber-100/70 flex items-center justify-between gap-2.5";
const CARD_TITLE: &str =
    "text-sm font-semibold text-zinc-800 group-hover:text-amber-900 transition-colors truncate";
const ACCENT_BAR: &str = "w-1.5 h-4 rounded-full bg-amber-400 shrink-0";

const META_ROW: &str = "flex items-center gap-1.5 text-xs font-medium text-zinc-500";
const META_ROW_MUTED: &str = "flex items-center gap-1.5 text-xs text-zinc-400";
const ICON_SM: &str = "w-3.5 h-3.5 shrink-0 text-amber-600/70";
const ICON_SM_MUTED: &str = "w-3.5 h-3.5 shrink-0 text-zinc-300";

// Touch-friendly status pill & filter tab tokens
const PILL: &str =
    "inline-flex items-center justify-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium shrink-0 transition-all duration-150";
const PILL_AMBER: &str = "bg-amber-100/80 text-amber-800 border border-amber-200/50";
const PILL_ZINC: &str = "bg-zinc-100 text-zinc-500 border border-zinc-200/50";

// Creation card with responsive min-height matching DashboardCard
const CTA_DASHED: &str =
    "flex flex-col items-center justify-center gap-2 min-h-[148px] h-full p-4 rounded-2xl border-2 border-dashed border-amber-300/80 bg-amber-50/20 text-amber-600 hover:text-amber-700 hover:border-amber-400 hover:bg-amber-50/60 transition-all duration-200 cursor-pointer group focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-400";

const FOCUS_RING: &str =
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-400 focus-visible:ring-offset-1 rounded-xl";

// ─────────────────────────────────────────────
//  Sorting options
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum SortOption {
    EditedDesc,
    NameAsc,
    CreatedDesc,
}

// ─────────────────────────────────────────────
//  Dashboard Component
// ─────────────────────────────────────────────

#[component]
pub fn Dashboard() -> Element {
    console::debug_1(&"Rendering Dashboard...".into());
    let storage = use_context::<Signal<StorageManager>>();
    let toasts = use_context::<Signal<Vec<ToastMessage>>>();

    let mut cook_and_run_list = use_resource(move || async move {
        console::debug_1(&"Loading cook and run list...".into());
        let storage = storage.read();
        match storage.select_cook_and_run_meta_list().await {
            Ok(list) => Ok(list),
            Err(err) => {
                console::error_1(&format!("Failed to load cook and run list: {}", err).into());
                trigger_error_toast(toasts, "Loading error", "Failed to load data!");
                Err("Failed to load data!".to_string())
            }
        }
    });

    let mut create_project_signal = use_signal(|| false);
    let mut search_signal = use_signal(String::new);
    let mut sort_signal = use_signal(|| SortOption::EditedDesc);

    let list_state = cook_and_run_list.read_unchecked();
    let query = search_signal.read().trim().to_lowercase();

    let header_count: String = match &*list_state {
        Some(Ok(list)) => {
            let total = list.len();
            if query.is_empty() {
                format!("{total} {}", if total == 1 { "project" } else { "projects" })
            } else {
                let matched = list
                    .iter()
                    .filter(|c| c.name.to_lowercase().contains(&query))
                    .count();
                format!("{matched} of {total} projects")
            }
        }
        _ => String::new(),
    };

    let has_content = matches!(&*list_state, Some(Ok(list)) if !list.is_empty());
    let total_is_empty = matches!(&*list_state, Some(Ok(list)) if list.is_empty());

    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8 space-y-6 sm:space-y-8",

            // ── Page Header Section ──────────────────────────────────
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-amber-100/80 pb-4",
                div { class: "flex items-baseline gap-3",
                    Headline1 { headline: "Projects" }
                    if !header_count.is_empty() {
                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold bg-amber-100/80 text-amber-800 border border-amber-200/60",
                            "{header_count}"
                        }
                    }
                }

                // Header Action Button (Mobile & Desktop)
                button {
                    r#type: "button",
                    class: "inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-amber-500 hover:bg-amber-600 text-white font-semibold text-sm shadow-xs hover:shadow transition-all duration-150 active:scale-[0.98] {FOCUS_RING}",
                    onclick: move |_| create_project_signal.set(true),
                    span { class: "text-lg leading-none font-bold", "+" }
                    span { "New Project" }
                }
            }

            // ── Controls Toolbar: Live Search & Segmented Sort ────────
            if has_content {
                div { class: "bg-white/80 backdrop-blur-xs p-3 sm:p-4 rounded-2xl border border-amber-100 shadow-xs flex flex-col sm:flex-row sm:items-center justify-between gap-3 sm:gap-4",

                    // Search input bar
                    div { class: "relative w-full sm:max-w-xs md:max-w-sm",
                        svg {
                            class: "absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-amber-600/60 pointer-events-none",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            path { d: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" }
                        }
                        input {
                            r#type: "text",
                            class: "{NATIVE_INPUT} pl-9 pr-3",
                            placeholder: "Search projects…",
                            aria_label: "Search projects",
                            value: "{search_signal.read()}",
                            oninput: move |e: Event<FormData>| search_signal.set(e.value()),
                        }
                    }

                    // Sort Pills (Segmented Tabs)
                    div { class: "flex items-center justify-start sm:justify-end gap-1.5 bg-amber-50/80 p-1.5 rounded-xl border border-amber-100/80 overflow-x-auto w-full sm:w-auto shrink-0",
                        span { class: "text-[11px] font-semibold tracking-wider text-amber-900/60 uppercase px-2 shrink-0 hidden md:inline",
                            "Sort:"
                        }
                        button {
                            r#type: "button",
                            class: if *sort_signal.read() == SortOption::EditedDesc { "px-3 py-1.5 rounded-lg text-xs font-semibold bg-white text-amber-900 shadow-xs border border-amber-200/60 transition-all {FOCUS_RING}" } else { "px-3 py-1.5 rounded-lg text-xs font-medium text-zinc-600 hover:text-zinc-900 hover:bg-amber-100/50 transition-all {FOCUS_RING}" },
                            onclick: move |_| sort_signal.set(SortOption::EditedDesc),
                            "Recent"
                        }
                        button {
                            r#type: "button",
                            class: if *sort_signal.read() == SortOption::NameAsc { "px-3 py-1.5 rounded-lg text-xs font-semibold bg-white text-amber-900 shadow-xs border border-amber-200/60 transition-all {FOCUS_RING}" } else { "px-3 py-1.5 rounded-lg text-xs font-medium text-zinc-600 hover:text-zinc-900 hover:bg-amber-100/50 transition-all {FOCUS_RING}" },
                            onclick: move |_| sort_signal.set(SortOption::NameAsc),
                            "Name"
                        }
                        button {
                            r#type: "button",
                            class: if *sort_signal.read() == SortOption::CreatedDesc { "px-3 py-1.5 rounded-lg text-xs font-semibold bg-white text-amber-900 shadow-xs border border-amber-200/60 transition-all {FOCUS_RING}" } else { "px-3 py-1.5 rounded-lg text-xs font-medium text-zinc-600 hover:text-zinc-900 hover:bg-amber-100/50 transition-all {FOCUS_RING}" },
                            onclick: move |_| sort_signal.set(SortOption::CreatedDesc),
                            "Created"
                        }
                    }
                }
            }

            // ── Grid / Empty State ────────────────────────────────────
            if total_is_empty {
                div { class: "flex flex-col items-center justify-center text-center gap-5 py-16 sm:py-24 px-4 bg-white/50 rounded-3xl border border-dashed border-amber-200/80 max-w-lg mx-auto my-8",
                    div { class: "w-16 h-16 rounded-2xl bg-amber-100/80 border border-amber-200 flex items-center justify-center shadow-xs",
                        svg {
                            class: "w-8 h-8 text-amber-600",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M4.083 9h1.946c.089 0 .173.024.25.067l1.591 1.591c.114.114.267.184.43.184h3.4a.6.6 0 00.43-.184l1.591-1.591a.6.6 0 01.25-.067h1.946M4.083 9L2.145 5.5A2 2 0 013.87 3h12.26a2 2 0 011.725 2.5L15.917 9M4.083 9L2 15a2 2 0 002 2h12a2 2 0 002-2l-2.083-6" }
                        }
                    }
                    div { class: "space-y-1.5 max-w-xs",
                        h2 { class: "text-lg font-bold text-zinc-900", "No projects yet" }
                        p { class: "text-sm text-zinc-500 leading-relaxed",
                            "Create your first project to start mapping routes, courses and costs."
                        }
                    }
                    button {
                        r#type: "button",
                        class: "w-64 py-3 px-4 rounded-xl bg-amber-500 hover:bg-amber-600 text-white font-semibold text-sm shadow-xs transition-all flex items-center justify-center gap-2 {FOCUS_RING}",
                        onclick: move |_| {
                            create_project_signal.set(true);
                        },
                        span { class: "text-lg leading-none font-bold", "+" }
                        span { "Create Project" }
                    }
                }
            } else {
                div { class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 sm:gap-5",

                    match &*list_state {
                        Some(Err(message)) => rsx! {
                            div { class: "col-span-full flex flex-col items-center gap-3 py-12 text-center bg-white rounded-2xl border border-amber-100 p-6",
                                svg {
                                    class: "w-9 h-9 text-amber-500",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l6.28 11.18c.75 1.334-.213 2.98-1.744 2.98H3.72c-1.53 0-2.493-1.646-1.744-2.98l6.28-11.18zM11 14a1 1 0 11-2 0 1 1 0 012 0zm-1-3a1 1 0 01-1-1V7a1 1 0 112 0v3a1 1 0 01-1 1z" }
                                }
                                p { class: "text-sm text-zinc-600 font-medium", "{message}" }
                                button {
                                    r#type: "button",
                                    class: "text-sm font-semibold text-amber-700 hover:text-amber-800 underline underline-offset-4 {FOCUS_RING}",
                                    onclick: move |_| {
                                        cook_and_run_list.restart();
                                    },
                                    "Try again"
                                }
                            }
                        },
                        Some(Ok(list)) => {
                            let mut items: Vec<_> = list
                                .iter()
                                .filter(|c| query.is_empty() || c.name.to_lowercase().contains(&query))
                                .collect();

                            match *sort_signal.read() {
                                SortOption::NameAsc => {
                                    items
                                        .sort_by(|a, b| {
                                            a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                        })
                                }
                                SortOption::CreatedDesc => {
                                    items.sort_by(|a, b| b.created.cmp(&a.created))
                                }
                                SortOption::EditedDesc => items.sort_by(|a, b| b.edited.cmp(&a.edited)),
                            }
                            if items.is_empty() {
                                let empty_message = if query.is_empty() {
                                    "No projects yet — create your first one below.".to_string()
                                } else {
                                    format!("No projects match '{query}'.")
                                };
                                rsx! {
                                    div { class: "col-span-full text-center py-8 bg-amber-50/40 rounded-2xl border border-amber-100/60",
                                        p { class: "text-sm text-zinc-500 italic", "{empty_message}" }
                                    }
                                }
                            } else {
                                rsx! {
                                    {
                                        items
                                            .into_iter()
                                            .map(|cook_and_run| {
                                                let created_fmt = cook_and_run
                                                    .created
                                                    .format("%d.%m.%Y · %H:%M")
                                                    .to_string();
                                                let edited_fmt = if cook_and_run.edited != cook_and_run.created {
                                                    Some(cook_and_run.edited.format("%d.%m.%Y · %H:%M").to_string())
                                                } else {
                                                    None
                                                };
                                                rsx! {
                                                    DashboardCard {
                                                        key: "{cook_and_run.id}",
                                                        id: cook_and_run.id,
                                                        name: cook_and_run.name.clone(),
                                                        created: created_fmt,
                                                        edited: edited_fmt,
                                                        uploaded: cook_and_run.is_in_cloud,
                                                    }
                                                }
                                            })
                                    }
                                }
                            }
                        }
                        None => rsx! {
                            LoadingCard {}
                            LoadingCard {}
                            LoadingCard {}
                        },
                    }

                    // ── Inline Add Project Card ──────────────────────
                    button {
                        r#type: "button",
                        class: "{CTA_DASHED} {FOCUS_RING}",
                        onclick: move |_| {
                            create_project_signal.set(true);
                        },
                        div { class: "w-9 h-9 rounded-full border-2 border-current flex items-center justify-center text-xl font-bold leading-none group-hover:scale-110 transition-transform duration-150",
                            "+"
                        }
                        div { class: "text-center space-y-0.5",
                            span { class: "block text-sm font-semibold tracking-wide",
                                "New Project"
                            }
                            span { class: "block text-xs text-amber-700/70 font-normal",
                                "Start planning a route"
                            }
                        }
                    }
                }
            }
        }

        // ── Create Project Dialog ──────────────────────────────────
        if *create_project_signal.read() {
            CreateProjectDialog { create_project_signal }
        }
    }
}

// ─────────────────────────────────────────────
//  Dashboard Card Component
// ─────────────────────────────────────────────

#[derive(PartialEq, Props, Clone)]
struct DashboardCardProps {
    id: Uuid,
    name: String,
    created: String,
    edited: Option<String>,
    uploaded: bool,
}

#[component]
fn DashboardCard(props: DashboardCardProps) -> Element {
    rsx! {
        Link {
            to: Route::Overview {
                cook_and_run_id: props.id,
            },
            class: "{CARD} {CARD_INTERACTIVE} {FOCUS_RING}",
            aria_label: "Open project {props.name}",

            // Header stripe
            div { class: "{CARD_HEADER}",
                div { class: "flex items-center gap-2 min-w-0 flex-1",
                    div { class: "{ACCENT_BAR}" }
                    span { class: "{CARD_TITLE}", "{props.name}" }
                }

                // Cloud status
                div {
                    class: if props.uploaded { "{PILL} {PILL_AMBER}" } else { "{PILL} {PILL_ZINC}" },
                    title: if props.uploaded { "Saved to cloud" } else { "Stored locally only" },
                    svg {
                        class: "w-3 h-3 shrink-0",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M16.88 9.94a5 5 0 00-9.72-1.47A4 4 0 006 17h9a4 4 0 001.88-7.06z" }
                    }
                    span {
                        if props.uploaded {
                            "Cloud"
                        } else {
                            "Local"
                        }
                    }
                }
            }

            // Card body (flex-1 ensures height consistency across grid items)
            div { class: "px-4 py-3.5 space-y-2 flex-1 flex flex-col justify-end",

                // Created
                div { class: "{META_ROW}",
                    svg {
                        class: "{ICON_SM}",
                        fill: "currentColor",
                        view_box: "0 0 20 20",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zM2 9v7a2 2 0 002 2h12a2 2 0 002-2V9H2z" }
                    }
                    span { "Created {props.created}" }
                }

                // Edited
                match &props.edited {
                    Some(edited) => rsx! {
                        div { class: "{META_ROW_MUTED}",
                            svg {
                                class: "{ICON_SM_MUTED}",
                                fill: "currentColor",
                                view_box: "0 0 20 20",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M17.414 2.586a2 2 0 010 2.828l-8.586 8.586a2 2 0 01-.879.515l-4 1a1 1 0 01-1.213-1.213l1-4a2 2 0 01.515-.879l8.586-8.586a2 2 0 012.828 0zM15 5l-1-1L6 12l-.5 2 .5.5 2-.5L15 5z" }
                            }
                            span { "Edited {edited}" }
                        }
                    },
                    None => rsx! {},
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Loading Skeleton Card
// ─────────────────────────────────────────────

#[component]
fn LoadingCard() -> Element {
    rsx! {
        div { class: "bg-white rounded-2xl border border-amber-100 shadow-xs overflow-hidden flex flex-col justify-between h-full min-h-[148px]",
            div { class: "px-4 py-3 bg-amber-50/60 border-b border-amber-100/70 flex items-center justify-between",
                div { class: "h-4 w-28 bg-amber-200/60 rounded-full animate-pulse" }
                div { class: "h-5 w-14 bg-amber-100 rounded-full animate-pulse" }
            }
            div { class: "px-4 py-3.5 space-y-2.5 flex-1 flex flex-col justify-end",
                div { class: "flex items-center gap-1.5",
                    div { class: "w-3.5 h-3.5 rounded-full bg-amber-200/50 animate-pulse shrink-0" }
                    div { class: "h-3.5 w-32 bg-zinc-200/70 rounded-full animate-pulse" }
                }
                div { class: "flex items-center gap-1.5",
                    div { class: "w-3.5 h-3.5 rounded-full bg-zinc-200/40 animate-pulse shrink-0" }
                    div { class: "h-3 w-24 bg-zinc-100 rounded-full animate-pulse" }
                }
            }
        }
        span { class: "sr-only", "Loading…" }
    }
}

// ─────────────────────────────────────────────
//  Create Project Modal Dialog
// ─────────────────────────────────────────────

#[component]
fn CreateProjectDialog(create_project_signal: Signal<bool>) -> Element {
    let mut project_name_signal = use_signal(|| "".to_string());
    let mut error_name_signal = use_signal(|| "".to_string());
    let mut error_signal = use_signal(|| "".to_string());

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6 bg-black/40 backdrop-blur-xs overflow-y-auto animate-in fade-in duration-200",
            div { class: "relative bg-white rounded-3xl border border-amber-100 shadow-2xl w-full max-w-md overflow-hidden my-auto",

                // Header
                div { class: "px-6 py-4.5 bg-amber-50/80 border-b border-amber-100/80 flex items-center justify-between",
                    div { class: "flex items-center gap-2.5",
                        div { class: "w-1.5 h-5 rounded-full bg-amber-400" }
                        h3 { class: "text-base font-bold text-zinc-900", "New Project" }
                    }
                    CloseButton {
                        onclick: move |_| {
                            create_project_signal.set(false);
                        },
                    }
                }

                // Form Container
                div { class: "p-6 space-y-5",

                    div { class: "space-y-1.5",
                        label { class: "{LBL}", "Project Name" }
                        Input {
                            place_holer: Some("e.g. Summer Cook & Run 2025".to_string()),
                            value: project_name_signal,
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

                    div { class: "flex items-center gap-3 pt-2",
                        SecondaryButton {
                            text: "Upload".to_string(),
                            action: async_action!(
                                { if let Some(doc) = web_sys::window().and_then(| w | w.document()) { if let
                                Some(el) = doc.get_element_by_id("project_upload") { if let Ok(input) = el
                                .dyn_into::< HtmlInputElement > () { input.click(); } } } }
                            ),
                        }
                        ConfirmButton {
                            text: "Create".to_string(),
                            error_signal,
                            action: async_action!(
                                { if project_name_signal.read().trim().is_empty() { error_name_signal
                                .set("Project name cannot be empty!".to_string()); return; } let project_id =
                                Uuid::new_v4(); let mut storage = use_context::< Signal < StorageManager >> ();
                                let mut storage = storage.write(); let cook_and_run = CookAndRunCreate { name :
                                project_name_signal.read().to_string(), }; let result = storage
                                .create_cook_and_run(project_id, & cook_and_run). await; if let Err(e) = result {
                                console::error_1(& format!("Error creating project: {}", e) .into(),);
                                error_signal.set("Creating project failed!".to_string()); return; }
                                create_project_signal.set(false); }
                            ),
                        }
                    }
                }
            }
        }
    }
}