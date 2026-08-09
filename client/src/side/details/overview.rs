use async_std::task::sleep;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Utc};
use dioxus::prelude::*;
use js_sys::Date;
use std::time::Duration;
use uuid::Uuid;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, js_sys, Blob, HtmlAnchorElement, Url};

use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::AsyncAction;
use crate::storage::{CookAndRunMetaData, CookAndRunMetaUpdate, StorageManager};
use crate::{async_action, Route};

// Design-System UI Imports
use crate::ui::{
    buttons::{ConfirmButton, PrimaryButton, SecondaryButton, WarnButton},
    cards::{BaseCard, CardHeader, InfoCard},
    dialogs::Modal,
    forms::{Input, InputDate, InputError},
    icons::{CloudIcon, DeviceIcon, DownloadIcon, ExportIcon, TrashIcon, UploadIcon},
    tokens::SAVE_GLOW_CSS,
    typography::{FieldLabel, Headline1, Text},
};

// ─────────────────────────────────────────────
//  Async Helpers
// ─────────────────────────────────────────────

async fn delete_cook_and_run_project(id: Uuid) -> Result<(), String> {
    let mut storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();
    storage.delete_cook_and_run(id).await
}

async fn update_meta_of_cook_and_run(
    id: Uuid,
    new_name: String,
    occur: DateTime<Utc>,
) -> Result<(), String> {
    let mut storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();
    storage
        .update_meta_of_cook_and_run(
            id,
            &CookAndRunMetaUpdate {
                name: new_name,
                occur,
            },
        )
        .await
}

async fn upload_cook_and_run(id: Uuid) -> Result<Uuid, String> {
    let mut storage: Signal<StorageManager> = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();
    storage.upload_to_cloud(id).await
}

async fn download_cook_and_run(id: Uuid) -> Result<Uuid, String> {
    let mut storage: Signal<StorageManager> = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();
    storage.download_from_cloud(id).await
}

async fn export_file(id: Uuid) -> Result<(), String> {
    let storage = use_context::<Signal<StorageManager>>();
    let storage = storage.read().clone();
    let cook_and_run = storage.select_cook_and_run(id).await?;

    let json = serde_json::to_string(&cook_and_run)
        .map_err(|e| format!("Could not parse Cook and Run: {}", e))?;

    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(&json));
    let blob = Blob::new_with_str_sequence(&array).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    a.set_href(&url);
    a.set_download(&format!("{}.tcc", cook_and_run.name));
    document.body().unwrap().append_child(&a).unwrap();
    a.click();
    document.body().unwrap().remove_child(&a).unwrap();
    Url::revoke_object_url(&url).unwrap();
    Ok(())
}

// ─────────────────────────────────────────────
//  Root Component
// ─────────────────────────────────────────────

#[component]
pub fn Overview(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let cook_and_run: Resource<Result<CookAndRunMetaData, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let cook_and_run = storage.select_cook_and_run_meta(cook_and_run_id).await?;
            Ok(cook_and_run)
        }
    });

    match &*cook_and_run.clone().read() {
        None => rsx!(
            LoadingPage {}
        ),
        Some(Err(e)) => rsx!(
            ErrorPage {
                error_text: "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
                error_details: e.clone(),
            }
        ),
        Some(Ok(meta)) => rsx!(
            OverviewContent { cook_and_run_meta: meta.clone() }
        ),
    }
}

// ─────────────────────────────────────────────
//  Content Component
// ─────────────────────────────────────────────

#[component]
pub fn OverviewContent(cook_and_run_meta: CookAndRunMetaData) -> Element {
    let navigator = use_navigator();
    let mut delete_dialog_signal = use_signal(|| false);
    let mut error_name_signal = use_signal(|| "".to_string());
    let mut error_signal = use_signal(|| "".to_string());
    let mut name_signal = use_signal(|| cook_and_run_meta.name.clone());
    let mut occur_signal = use_signal(|| cook_and_run_meta.occur);

    // keep originals to avoid moving fields out of `cook_and_run_meta` when
    // captured by multiple closures
    let original_name = cook_and_run_meta.name.clone();
    let original_occur = cook_and_run_meta.occur;
    let original_name_for_date = original_name.clone();
    let original_occur_for_date = original_occur.clone();

    let mut save_success_signal = use_signal(|| false);
    let mut has_unsaved_changes = use_signal(|| false);

    let mut action_loading_signal = use_signal(|| false);
    let mut delete_loading_signal = use_signal(|| false);

    let on_name_input = move |evt: FormEvent| {
        let current_name = evt.value();
        name_signal.set(current_name.clone());

        if !error_name_signal.read().is_empty() {
            error_name_signal.set("".to_string());
        }

        let is_changed = current_name != original_name
            || *occur_signal.read() != original_occur;
        has_unsaved_changes.set(is_changed);
    };

    let on_date_input = move |evt: FormEvent| {
        if let Ok(date) = NaiveDate::parse_from_str(&evt.value(), "%Y-%m-%d") {
            let time = occur_signal.read().time();
            let new_occur = date.and_time(time).and_utc();
            occur_signal.set(new_occur);

            let is_changed = name_signal.read().clone() != original_name_for_date
                || new_occur != original_occur_for_date;
            has_unsaved_changes.set(is_changed);
        }
    };

    let is_success = *save_success_signal.read();
    let can_save = *has_unsaved_changes.read();

    let card_border_class = if is_success {
        "save-glow-card"
    } else {
        ""
    };

    let header_class = if is_success {
        "save-glow-header"
    } else {
        ""
    };

    let project_id = cook_and_run_meta.id;
    let is_cloud = cook_and_run_meta.is_in_cloud;

    rsx! {
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        section { class: "max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8",
            // Page Header
            Headline1 {
                headline: "Project Overview",
                subtitle: Some(
                    "Manage your project settings, cloud synchronization, and data export."
                        .to_string(),
                ),
            }

            // Grid Container: 2 Spalten auf Desktop (lg), 1 Spalte auf Mobile
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6 items-start",

                // Linke Spalte (Haupt-Einstellungen & Speicherverwaltung - 2/3 Breite)
                div { class: "lg:col-span-2 space-y-6",

                    // Projekt-Formular Karte
                    BaseCard { class: card_border_class.to_string(),
                        CardHeader {
                            title: "Project Settings".to_string(),
                            subtitle: Some("Update the basic information of your cooking event.".to_string()),
                            class: header_class.to_string(),
                        }

                        div { class: "p-6 space-y-5",
                            // Projektname Input
                            div {
                                FieldLabel { text: "Project Name".to_string() }
                                Input {
                                    place_holer: Some("e.g. Summer Cooking 2026".to_string()),
                                    value: name_signal.read().clone(),
                                    is_error: !error_name_signal.read().is_empty(),
                                    oninput: on_name_input,
                                }
                                InputError { error: error_name_signal.read().clone() }
                            }

                            // Datum Input
                            div {
                                FieldLabel { text: "Event Date".to_string() }
                                InputDate {
                                    value: occur_signal.read().format("%Y-%m-%d").to_string(),
                                    oninput: on_date_input,
                                }
                            }

                            InputError { error: error_signal.read().clone() }

                            // Action Bar
                            div { class: "flex items-center justify-end pt-4 border-t border-amber-100/60",
                                div { class: if can_save { "" } else { "opacity-50 pointer-events-none" },
                                    ConfirmButton {
                                        text: if is_success { "Saved!".to_string() } else { "Save Changes".to_string() },
                                        error_signal: Some(error_signal),
                                        action: async_action!(
                                            { if name_signal.read().trim().is_empty() { error_name_signal
                                            .set("Project name cannot be empty!".to_string()); return; } let result =
                                            update_meta_of_cook_and_run(project_id, name_signal.read().clone(), *
                                            occur_signal.read(),). await; if let Err(e) = result { console::error_1(&
                                            format!("Error updating meta: {}", e) .into()); error_signal
                                            .set("Failed to update project settings!".to_string()); } else {
                                            has_unsaved_changes.set(false); save_success_signal.set(true); spawn(async move {
                                            sleep(Duration::from_secs(2)). await; save_success_signal.set(false); }); } }
                                        ),
                                    }
                                }
                            }
                        }
                    }

                    // Daten- & Speicherverwaltung Karte
                    BaseCard {
                        CardHeader {
                            title: "Data Management & Storage".to_string(),
                            subtitle: Some("Sync, export or permanently remove this project.".to_string()),
                        }

                        div { class: "p-6 space-y-6",
                            // Status Anzeige
                            div { class: "flex items-center justify-between p-4 bg-amber-50/50 rounded-xl border border-amber-100",
                                div { class: "flex items-center gap-3",
                                    if is_cloud {
                                        div { class: "p-2 bg-amber-100 text-amber-700 rounded-lg shrink-0",
                                            CloudIcon { class: Some("w-5 h-5".to_string()) }
                                        }
                                        div {
                                            p { class: "text-sm font-semibold text-zinc-800",
                                                "Cloud Project"
                                            }
                                            p { class: "text-xs text-zinc-500", "Synced with server" }
                                        }
                                    } else {
                                        div { class: "p-2 bg-zinc-100 text-zinc-700 rounded-lg shrink-0",
                                            DeviceIcon { class: Some("w-5 h-5".to_string()) }
                                        }
                                        div {
                                            p { class: "text-sm font-semibold text-zinc-800",
                                                "Local Project"
                                            }
                                            p { class: "text-xs text-zinc-500",
                                                "Stored on this device"
                                            }
                                        }
                                    }
                                }
                            }

                            // Aktionen Grid
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3 pt-2",
                                if is_cloud {
                                    SecondaryButton {
                                        text: "Download Local Copy".to_string(),
                                        icon: Some(rsx! {
                                            DownloadIcon { class: Some("w-4 h-4 text-zinc-600".to_string()) }
                                        }),
                                        action: async_action!(
                                            { action_loading_signal.set(true); if let Err(e) =
                                            download_cook_and_run(project_id). await { console::error_1(&
                                            format!("Error downloading project: {}", e) .into()); } action_loading_signal
                                            .set(false); }
                                        ),
                                    }
                                } else {
                                    PrimaryButton {
                                        text: "Upload to Cloud".to_string(),
                                        icon: Some(rsx! {
                                            UploadIcon { class: Some("w-4 h-4 text-white".to_string()) }
                                        }),
                                        action: async_action!(
                                            { action_loading_signal.set(true); if let Err(e) =
                                            upload_cook_and_run(project_id). await { console::error_1(&
                                            format!("Error uploading project: {}", e) .into()); } action_loading_signal
                                            .set(false); }
                                        ),
                                    }
                                }

                                SecondaryButton {
                                    text: "Export File (.tcc)".to_string(),
                                    icon: Some(rsx! {
                                        ExportIcon { class: Some("w-4 h-4 text-zinc-600".to_string()) }
                                    }),
                                    action: async_action!(
                                        { if let Err(e) = export_file(project_id). await { console::error_1(&
                                        format!("Error exporting file: {}", e) .into()); } }
                                    ),
                                }
                            }

                            // Lösch-Bereich (Danger Zone)
                            div { class: "pt-4 border-t border-red-100 flex items-center justify-between gap-4",
                                div {
                                    p { class: "text-xs font-semibold text-red-700 uppercase tracking-wider",
                                        "Danger Zone"
                                    }
                                    p { class: "text-xs text-zinc-500",
                                        "Permanently remove this project"
                                    }
                                }
                                WarnButton {
                                    text: "Delete Project".to_string(),
                                    icon: Some(rsx! {
                                        TrashIcon { class: Some("w-4 h-4 text-white".to_string()) }
                                    }),
                                    onclick: move |_| delete_dialog_signal.set(true),
                                }
                            }
                        }
                    }
                }

                // Rechte Spalte (Info-Karte - 1/3 Breite)
                div { class: "lg:col-span-1",
                    if is_cloud {
                        InfoCard {
                            title: "Cloud Storage Info".to_string(),
                            lines: vec![
                                "This project is stored in the cloud.".to_string(),
                                "Your data is synced across devices and backed up automatically.".to_string(),
                                "Offline work is not available. Use the Download button to create a local copy."
                                    .to_string(),
                            ],
                        }
                    } else {
                        InfoCard {
                            title: "Local Storage Info".to_string(),
                            lines: vec![
                                "This project is saved only on this device.".to_string(),
                                "You can work offline anytime without an internet connection.".to_string(),
                                "Upload to the cloud to access this project from other devices.".to_string(),
                            ],
                        }
                    }
                }
            }

            // Modal: Projekt löschen
            if *delete_dialog_signal.read() {
                Modal {
                    title: "Delete Project".to_string(),
                    on_close: move |_| delete_dialog_signal.set(false),
                    children: rsx! {
                        div { class: "p-6 space-y-5",
                            Text {
                                text: "Are you sure you want to delete this project? This will permanently remove all data, including teams, locations, and courses. This action cannot be undone."
                                    .to_string(),
                                class: Some("text-zinc-600 leading-relaxed".to_string()),
                            }

                            div { class: "flex items-center justify-end gap-3 pt-4 border-t border-zinc-100",
                                SecondaryButton {
                                    text: "Cancel".to_string(),
                                    onclick: move |_| delete_dialog_signal.set(false),
                                }
                                WarnButton {
                                    text: if *delete_loading_signal.read() { "Deleting...".to_string() } else { "Delete Permanently".to_string() },
                                    action: async_action!(
                                        { delete_loading_signal.set(true); let result =
                                        delete_cook_and_run_project(project_id). await; delete_loading_signal.set(false);
                                        if let Err(e) = result { console::error_1(& format!("Error deleting project: {}",
                                        e) .into()); } else { delete_dialog_signal.set(false); navigator
                                        .push(Route::Dashboard {}); } }
                                    ),
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}