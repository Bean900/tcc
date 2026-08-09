use chrono::{DateTime, Local, NaiveDateTime, Utc};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::{
    async_action,
    side::{AsyncAction, ConfirmButton},
    storage::{CookAndRunCreate, StorageManager},
    trigger_error_toast,
    ui::{
        buttons::{PrimaryButton, SecondaryButton},
        cards::{BaseCard, CardHeader, SearchFilterCard},
        dialogs::Modal,
        forms::{Input, InputError},
        icons::{CloudIcon, DeviceIcon, PlusIcon},
        typography::{CaptionText, FieldLabel, Headline1, Text},
    },
    Route, ToastMessage,
};

// ─────────────────────────────────────────────
//  Shared Helpers
// ─────────────────────────────────────────────

fn relative_time(utc_dt: DateTime<Utc>) -> String {
    let local_dt = utc_dt.with_timezone(&Local);
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
    fn from_edited(edited_utc: DateTime<Utc>) -> Self {
        Self {
            relative: relative_time(edited_utc),
            absolute: edited_utc.with_timezone(&Local).format("%d.%m.%Y · %H:%M").to_string(),
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
    fn from_occur(occur_utc: DateTime<Utc>) -> Self {
        let occur_date = occur_utc.date_naive();
        let today = Utc::now().date_naive();
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
pub enum SortOption {
    EditedDesc,
    NameAsc,
    CreatedDesc,
    OccureDesc,
}

// ─────────────────────────────────────────────
//  Dashboard Component
// ─────────────────────────────────────────────

#[component]
pub fn Dashboard() -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let toasts = use_context::<Signal<Vec<ToastMessage>>>();

    // ── Component State Signals ───────
    let mut create_project_signal = use_signal(|| false);
    let mut search_signal = use_signal(String::new);
    let mut sort_signal = use_signal(|| SortOption::OccureDesc);

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

    let sort_options = vec![
        (SortOption::OccureDesc, "Occurrence Date".to_string()),
        (SortOption::EditedDesc, "Recently Edited".to_string()),
        (SortOption::CreatedDesc, "Recently Created".to_string()),
        (SortOption::NameAsc, "Name (A–Z)".to_string()),
    ];

    rsx! {
        // Breitere Maximalbreite für große Bildschirme (max-w-[1800px])
        div { class: "max-w-[1800px] mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8 space-y-6 sm:space-y-8",

            // ── Header Bar ──────────────────────────────────
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-amber-100/80 pb-2",
                div { class: "flex items-baseline gap-3",
                    Headline1 { headline: "Projects".to_string() }
                    if !header_count.is_empty() {
                        CaptionText {
                            text: header_count,
                            class: "inline-flex items-center px-2.5 py-0.5 rounded-full font-semibold bg-amber-100/80 !text-amber-800 border border-amber-200/60"
                                .to_string(),
                        }
                    }
                }
                PrimaryButton {
                    text: "New Project".to_string(),
                    icon: rsx! {
                        PlusIcon { class: "w-5 h-5 text-white".to_string() }
                    },
                    onclick: move |_| create_project_signal.set(true),
                }
            }

            // ── Controls Toolbar ───────────────────────────
            if has_content {
                SearchFilterCard {
                    search_value: search_signal.read().clone(),
                    search_placeholder: Some("Search projects…".to_string()),
                    on_search_change: move |val| search_signal.set(val),
                    sort_value: *sort_signal.read(),
                    sort_options,
                    on_sort_change: move |val| sort_signal.set(val),
                    use_card_wrapper: true,
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
                        SortOption::OccureDesc => filtered.sort_by(|a, b| b.occur.cmp(&a.occur)),
                    }
                    rsx! {
                        // Dynamisches Skalieren von 1 Spalte (Mobil) bis zu 5 Spalten (2xl / Ultrawide)
                        div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4 sm:gap-6",
                            for item in filtered {
                                {
                                    let edited = EditedInfo::from_edited(item.edited);
                                    let occur_status = OccurStatus::from_occur(item.occur);
                                    let item_id = item.id;
                                    let is_in_cloud = item.is_in_cloud;
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
                                                        div { class: "flex items-center gap-1.5 shrink-0",
                                                            if is_in_cloud {
                                                                span {
                                                                    class: "inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium bg-sky-100/80 text-sky-800 border border-sky-200/60",
                                                                    title: "Stored in Cloud",
                                                                    CloudIcon { class: "w-3.5 h-3.5 stroke-current".to_string() }
                                                                    "Cloud"
                                                                }
                                                            } else {
                                                                span {
                                                                    class: "inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium bg-zinc-100 text-zinc-600 border border-zinc-200/60",
                                                                    title: "Stored locally",
                                                                    DeviceIcon { class: "w-3.5 h-3.5 stroke-current".to_string() }
                                                                    "Local"
                                                                }
                                                            }
                                                            span { class: "inline-flex items-center justify-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium shrink-0 {occur_status.pill_class()}",
                                                                "{occur_status.label()}"
                                                            }
                                                        }
                                                    },
                                                }
                                                div { class: "p-4 space-y-2.5",
                                                    CaptionText {
                                                        text: format!("Event: {}", item.occur.format("%d.%m.%Y")),
                                                        class: "block text-zinc-500 font-medium".to_string(),
                                                    }
                                                    div { title: "{edited.absolute}",
                                                        CaptionText {
                                                            text: format!("Edited {}", edited.relative),
                                                            class: "block text-zinc-400".to_string(),
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
                }
                Some(Err(_)) => rsx! {
                    div { class: "p-8 text-center bg-red-50 rounded-2xl border border-red-200",
                        Text {
                            text: "Fehler beim Laden der Projekte.".to_string(),
                            class: "text-red-600 font-medium".to_string(),
                        }
                    }
                },
                None => rsx! {
                    // Auch beim Skeleton-Grid dieselben Breakpoints nutzen
                    div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4 sm:gap-6 animate-pulse",
                        for _ in 0..5 {
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
                            FieldLabel { text: "Project Name".to_string() }
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