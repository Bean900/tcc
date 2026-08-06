use chrono::{DateTime, Local, NaiveDateTime, Utc};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::{console};

use crate::{
    Route, ToastMessage, async_action, side::{
        AsyncAction, ConfirmButton, Headline1, SecondaryButton,
    }, storage::{CookAndRunCreate, StorageManager}, trigger_error_toast, ui::{ cards::{BaseCard, CardHeader}, dialogs::Modal, forms::{Input, InputError}},
};

// ─────────────────────────────────────────────
//  Shared Tokens & Helpers
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-wider uppercase text-amber-800/80 mb-1.5";
const NATIVE_INPUT: &str =
    "w-full h-10 px-3.5 py-2 rounded-xl border border-amber-200/90 bg-amber-50/30 text-sm text-zinc-800 placeholder-zinc-400 focus:outline-none focus:ring-2 focus:ring-amber-400/50 focus:border-amber-400 focus:bg-white transition-all duration-150";

const FOCUS_RING: &str =
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-amber-400 focus-visible:ring-offset-1 rounded-xl";

fn to_local(naive_utc: NaiveDateTime) -> DateTime<Local> {
    DateTime::<Utc>::from_naive_utc_and_offset(naive_utc, Utc).with_timezone(&Local)
}

fn relative_time(local_dt: DateTime<Local>) -> String {
    let diff = Local::now().signed_duration_since(local_dt);
    if diff.num_seconds() < 60 {
        "just now".to_string()
    } else if diff.num_minutes() < 60 {
        format!("{}m ago", diff.num_minutes())
    } else if diff.num_hours() < 24 {
        format!("{}h ago", diff.num_hours())
    } else if diff.num_days() < 7 {
        format!("{}d ago", diff.num_days())
    } else if diff.num_weeks() < 5 {
        format!("{}w ago", diff.num_weeks())
    } else {
        local_dt.format("%d.%m.%Y").to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct EditedInfo {
    relative: String,
    absolute: String,
}

impl EditedInfo {
    fn from_edited(edited_utc: NaiveDateTime) -> Self {
        let local = to_local(edited_utc);
        Self {
            relative: relative_time(local),
            absolute: local.format("%d.%m.%Y · %H:%M").to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OccurStatus {
    Past,
    Today,
    Upcoming,
}

impl OccurStatus {
    fn from_occur(occur_utc: NaiveDateTime) -> Self {
        let occur_date = to_local(occur_utc).date_naive();
        let today = Local::now().date_naive();
        if occur_date < today {
            OccurStatus::Past
        } else if occur_date == today {
            OccurStatus::Today
        } else {
            OccurStatus::Upcoming
        }
    }

    fn label(&self) -> &'static str {
        match self {
            OccurStatus::Past => "Past",
            OccurStatus::Today => "Today",
            OccurStatus::Upcoming => "Upcoming",
        }
    }

    fn pill_class(&self) -> &'static str {
        match self {
            OccurStatus::Past => "bg-zinc-100 text-zinc-500 border border-zinc-200/50",
            OccurStatus::Today => "bg-emerald-100/80 text-emerald-800 border border-emerald-200/50",
            OccurStatus::Upcoming => "bg-amber-100/80 text-amber-800 border border-amber-200/50",
        }
    }
}

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
    let storage = use_context::<Signal<StorageManager>>();
    let toasts = use_context::<Signal<Vec<ToastMessage>>>();

    // ── Component State Signals (Must be top-level) ───────
    let mut create_project_signal = use_signal(|| false);
    let mut search_signal = use_signal(String::new);
    let search_error_signal = use_signal(String::new);
    let mut sort_signal = use_signal(|| SortOption::EditedDesc);

    // Modal State Signals
    let mut project_name_signal = use_signal(String::new);
    let mut error_name_signal = use_signal(String::new);
    let mut error_signal = use_signal(String::new);

    let cook_and_run_list = use_resource(move || async move {
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

    let list_ref = cook_and_run_list.read();
    let query = search_signal.read().trim().to_lowercase();

    let header_count: String = match &*list_ref {
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

    let has_content = matches!(&*list_ref, Some(Ok(list)) if !list.is_empty());

    rsx! {
        div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8 space-y-6 sm:space-y-8",

            // ── Header Bar ──────────────────────────────────
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-amber-100/80 pb-2",
                div { class: "flex items-baseline gap-3",
                    Headline1 { headline: "Projects" }
                    if !header_count.is_empty() {
                        span { class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold bg-amber-100/80 text-amber-800 border border-amber-200/60",
                            "{header_count}"
                        }
                    }
                }
                button {
                    r#type: "button",
                    class: "inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-amber-500 hover:bg-amber-600 text-white font-semibold text-sm shadow-xs hover:shadow transition-all duration-150 active:scale-[0.98] {FOCUS_RING}",
                    onclick: move |_| create_project_signal.set(true),
                    span { class: "text-lg leading-none font-bold", "+" }
                    span { "New Project" }
                }
            }

            // ── Controls Toolbar ───────────────────────────
            if has_content {
                div { class: "bg-white/80 backdrop-blur-xs p-3 sm:p-4 rounded-2xl border border-amber-100 shadow-xs flex flex-col sm:flex-row sm:items-center justify-between gap-3 sm:gap-4",
                    div { class: "relative w-full sm:max-w-xs md:max-w-sm",
                        Input {
                            place_holer: "Search projects…".to_string(),
                            value: search_signal.read().to_string(),
                            is_error: !search_error_signal.read().is_empty(),
                            oninput: move |e: FormEvent| {
                                let input_value = e.value().clone();
                                search_signal.set(input_value);
                            },
                        }
                    }
                    div { class: "flex items-center gap-1.5 bg-amber-50/60 p-1 rounded-xl border border-amber-100/80 shrink-0 self-start sm:self-auto",
                        button {
                            r#type: "button",
                            class: if *sort_signal.read() == SortOption::EditedDesc { "px-3 py-1.5 text-xs font-semibold rounded-lg bg-white text-amber-900 shadow-xs border border-amber-200/50" } else { "px-3 py-1.5 text-xs font-medium rounded-lg text-zinc-600 hover:text-amber-800" },
                            onclick: move |_| sort_signal.set(SortOption::EditedDesc),
                            "Recently Edited"
                        }
                        button {
                            r#type: "button",
                            class: if *sort_signal.read() == SortOption::NameAsc { "px-3 py-1.5 text-xs font-semibold rounded-lg bg-white text-amber-900 shadow-xs border border-amber-200/50" } else { "px-3 py-1.5 text-xs font-medium rounded-lg text-zinc-600 hover:text-amber-800" },
                            onclick: move |_| sort_signal.set(SortOption::NameAsc),
                            "Name A–Z"
                        }
                    }
                }
            }

            // ── Content Grid / List ────────────────────────
            match &*list_ref {
                Some(Ok(list)) => {
                    let mut filtered: Vec<_> = list
                        .iter()
                        .filter(|c| query.is_empty() || c.name.to_lowercase().contains(&query))
                        .collect();
                    match *sort_signal.read() {
                        SortOption::EditedDesc => {
                            filtered.sort_by(|a, b| b.edited.cmp(&a.edited))
                        }
                        SortOption::NameAsc => {
                            filtered
                                .sort_by(|a, b| {
                                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                })
                        }
                        SortOption::CreatedDesc => filtered.sort_by(|a, b| b.id.cmp(&a.id)),
                    }
                    rsx! {
                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6",
                            // Create Card Action Button
                            div {
                                class: "flex flex-col items-center justify-center gap-2 min-h-[148px] h-full p-4 rounded-2xl border-2 border-dashed border-amber-300/80 bg-amber-50/20 text-amber-600 hover:text-amber-700 hover:border-amber-400 hover:bg-amber-50/60 transition-all duration-200 cursor-pointer group {FOCUS_RING}",
                                onclick: move |_| create_project_signal.set(true),
                                div { class: "w-10 h-10 rounded-full bg-amber-100 text-amber-700 flex items-center justify-center font-bold text-xl group-hover:scale-110 transition-transform",
                                    "+"
                                }
                                span { class: "text-sm font-semibold", "Create New Project" }
                            }

                            for item in filtered {
                                {
                                    let edited = EditedInfo::from_edited(item.edited);
                                    let occur_status = OccurStatus::from_occur(item.occur);
                                    let item_id = item.id;
                                    rsx! {
                                        Link {
                                            key: "{item_id}",
                                            to: Route::Overview {
                                                cook_and_run_id: item_id,
                                            },
                                            BaseCard {
                                                CardHeader {
                                                    title: item.name.clone(),
                                                    action: rsx! {
                                                        span { class: "inline-flex items-center justify-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium shrink-0 {occur_status.pill_class()}",
                                                            "{occur_status.label()}"
                                                        }
                                                    },
                                                }
                                                div { class: "p-4 space-y-2.5",
                                                    div { class: "flex items-center gap-1.5 text-xs font-medium text-zinc-500",
                                                        span { "Event: {item.occur.format(\"%d.%m.%Y\")}" }
                                                    }
                                                    div {
                                                        class: "flex items-center gap-1.5 text-xs text-zinc-400",
                                                        title: "{edited.absolute}",
                                                        span { "Edited {edited.relative}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(_)) => rsx! {
                    div { class: "p-8 text-center text-red-600 bg-red-50 rounded-2xl border border-red-200",
                        "Fehler beim Laden der Projekte."
                    }
                },
                None => rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 animate-pulse",
                        for _ in 0..3 {
                            div { class: "h-36 bg-amber-100/40 rounded-2xl" }
                        }
                    }
                },
            }

            // ── Create Project Modal ────────────────────────
            if *create_project_signal.read() {
                Modal {
                    title: "Create Project".to_string(),
                    on_close: move || create_project_signal.set(false),
                    div { class: "space-y-4",
                        div {
                            label { class: "{LBL}", "Project Name" }
                            Input {
                                place_holer: "e.g. Summer Cooking 2026".to_string(),
                                value: project_name_signal.read().to_string(),
                                is_error: !error_name_signal.read().is_empty(),
                                oninput: move |e: FormEvent| {
                                    let input_value = e.value().clone();
                                    project_name_signal.set(input_value);
                                },
                            }
                            InputError { error: error_name_signal.read().to_string() }
                        }
                        InputError { error: error_signal.read().to_string() }

                        div { class: "flex justify-end gap-3 pt-3 border-t border-amber-100",
                            SecondaryButton {
                                text: "Cancel".to_string(),
                                action: async_action!(create_project_signal.set(false)),
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
                                    console::error_1(& format!("Error creating project: {}", e) .into());
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
}