use async_std::task::sleep;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use dioxus::prelude::*;
use std::time::Duration;
use uuid::Uuid;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, js_sys, Blob, HtmlAnchorElement, Url};

use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::{AsyncAction, Headline1, InputDate};
use crate::storage::{CookAndRunMetaData, CookAndRunMetaUpdate, StorageManager};
use crate::{async_action, AuthState};

use crate::{
    side::{CloseButton, ConfirmButton, Input, InputError, SecondaryButton, WarnButton},
    Route,
};

// ─────────────────────────────────────────────
//  Shared tokens
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

// ─────────────────────────────────────────────
//  CSS: Keyframe-Animation für den grünen Glow
// ─────────────────────────────────────────────

const SAVE_GLOW_CSS: &str = r#"
@keyframes save-glow {
    0%   {
        border-color: #d1fae5;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    20%  {
        border-color: #22c55e;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.22),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    55%  {
        border-color: #16a34a;
        box-shadow: 0 0 0 5px rgba(34, 197, 94, 0.10),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
    100% {
        border-color: #bbf7d0;
        box-shadow: 0 0 0 0px rgba(34, 197, 94, 0),
                    0 1px 3px 0 rgba(0, 0, 0, 0.06);
    }
}

/* Karte pulsiert grün */
.save-glow-card {
    animation: save-glow 2s ease-in-out forwards;
}

/* Header-Bereich der Karte: grüner Hintergrund */
.save-glow-card .save-glow-header {
    background-color: rgba(240, 253, 244, 0.70) !important;
    border-bottom-color: #bbf7d0 !important;
    transition: background-color 0.4s ease, border-color 0.4s ease;
}

/* Akzent-Balken: grün */
.save-glow-card .save-glow-accent {
    background-color: rgba(34, 197, 94, 0.75) !important;
    transition: background-color 0.4s ease;
}

/* Header-Text: dunkelgrün */
.save-glow-card .save-glow-title {
    color: #166534 !important;
    transition: color 0.4s ease;
}
"#;

// ─────────────────────────────────────────────
//  Async helpers
// ─────────────────────────────────────────────

async fn delete_cook_and_run_project(id: Uuid) -> Result<(), String> {
    let mut storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.write();
    storage.delete_cook_and_run(id).await
}

async fn update_meta_of_cook_and_run(
    id: Uuid,
    new_name: String,
    occur: NaiveDateTime,
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
//  Root
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

    match &*cook_and_run.read_unchecked() {
        None => rsx!(LoadingPage {}),
        Some(Err(e)) => rsx!(ErrorPage {
            error_text:
                "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        }),
        Some(Ok(meta)) => rsx!(OverviewContent {
            cook_and_run_meta: meta.clone()
        }),
    }
}

// ─────────────────────────────────────────────
//  Content
// ─────────────────────────────────────────────

#[component]
pub fn OverviewContent(cook_and_run_meta: CookAndRunMetaData) -> Element {
    let storage_signal = use_context::<Signal<StorageManager>>();

    let mut delete_dialog_signal = use_signal(|| false);
    let mut error_name_signal = use_signal(|| "".to_string());
    let mut error_signal = use_signal(|| "".to_string());
    let mut name_signal = use_signal(|| cook_and_run_meta.name.clone());
    let mut occur_signal = use_signal(|| cook_and_run_meta.occur);

    // true  → grüner Glow aktiv (2 s nach erfolgreichem Speichern)
    let mut save_success_signal = use_signal(|| false);

    // true  → ungespeicherte Änderungen vorhanden → Save-Button aktiv
    // false → gespeichert / noch keine Änderung → Save-Button deaktiviert
    let mut has_unsaved_changes = use_signal(|| false);

    let on_name_input = move |evt: FormEvent| {
        let current_name = evt.value();
        name_signal.set(current_name.clone());
        has_unsaved_changes.set(true);
        if current_name.is_empty() {
            error_name_signal.set("Project name cannot be empty!".to_string());
        } else {
            error_name_signal.set("".to_string());
        }
    };

    let on_save: AsyncAction = async_action!({
        let current_name = name_signal.read().clone();
        if current_name.is_empty() {
            error_name_signal.set("Project name cannot be empty!".to_string());
        } else {
            let result = update_meta_of_cook_and_run(
                cook_and_run_meta.id,
                current_name,
                *occur_signal.read(),
            )
            .await;
            if let Err(e) = result {
                console::error_1(&format!("Error saving project: {}", e).into());
                error_signal.set("Saving failed! Try again later.".to_string());
            } else {
                error_signal.set("".to_string());
                // Erfolgreich gespeichert → Button deaktivieren + Glow starten
                has_unsaved_changes.set(false);
                save_success_signal.set(true);
                spawn(async move {
                    sleep(Duration::from_millis(2000)).await;
                    save_success_signal.set(false);
                });
            }
        }
    });

    let error_login_signal = use_signal(|| match storage_signal.read().get_auth_state() {
        Ok(AuthState::LoggedIn(_)) => "".to_string(),
        Ok(AuthState::Loading(_)) => {
            console::error_1(&format!("Auth state ist loading!").into());
            "Loading...!".to_string()
        }
        Ok(AuthState::LoggedOut) => {
            console::error_1(&format!("Auth state ist not logged out!").into());
            "Logged out!".to_string()
        }
        Ok(AuthState::Error(e)) => {
            console::error_1(&format!("Auth error: {}", e).into());
            "Error!".to_string()
        }
        Err(e) => {
            console::error_1(&format!("Error while getting auth state: {}", e).into());
            "Error whole loading auth state!".to_string()
        }
    });

    let is_cloud = cook_and_run_meta.is_in_cloud;
    let navigator = use_navigator();

    let is_success = *save_success_signal.read();
    let can_save = *has_unsaved_changes.read();

    // Karte: im Erfolgsfall `.save-glow-card` → triggert die @keyframes-Animation
    let card_class = if is_success {
        "bg-white rounded-2xl border overflow-hidden save-glow-card"
    } else {
        "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden"
    };

    // Header greift im Erfolgsfall auf .save-glow-header (CSS oben) zurück
    let header_class = if is_success {
        "px-5 py-3.5 border-b flex items-center gap-2.5 save-glow-header"
    } else {
        "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5"
    };

    let accent_class = if is_success {
        "w-1.5 h-5 rounded-full save-glow-accent"
    } else {
        "w-1.5 h-5 rounded-full bg-amber-400/70"
    };

    let title_class = if is_success {
        "text-sm font-semibold save-glow-title"
    } else {
        "text-sm font-semibold text-zinc-800"
    };

    // Save-Button-Wrapper: deaktiviert solange keine Änderungen vorhanden
    let save_btn_wrapper_class = if can_save {
        ""
    } else {
        "opacity-40 pointer-events-none cursor-not-allowed"
    };

    rsx! {
        // ── Keyframe-CSS einmalig einbinden ───────────────────────
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        section { class: "px-8 py-6 space-y-8",

            // ── Page header ───────────────────────────────────────
            Headline1 { headline: "Overview" }

            // ── Settings card ─────────────────────────────────────
            div { class: "{card_class}",

                div { class: "{header_class}",
                    div { class: "{accent_class}" }
                    span { class: "{title_class}",
                          "Project Settings"
                    }
                }

                div { class: "px-5 py-5 space-y-4",

                    // Name + Date side by side
                    div { class: "grid grid-cols-2 gap-4",

                        div {
                            label { class: "{LBL}", "Project Name" }
                            Input {
                                place_holer: Some("e.g. Summer Cook & Run 2025".to_string()),
                                value: name_signal.read(),
                                is_error: !error_name_signal.read().is_empty(),
                                oninput: on_name_input,
                            }
                            InputError { error: error_name_signal.read() }
                        }

                        div {
                            label { class: "{LBL}", "Occurring" }
                            InputDate {
                                value: occur_signal.read().format("%Y-%m-%d"),
                                oninput: move |e: FormEvent| {
                                    match NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                                        Ok(d) => {
                                            occur_signal.set(
                                                d.and_time(
                                                    NaiveTime::from_hms_opt(0, 0, 0)
                                                        .expect("Valid time"),
                                                ),
                                            );
                                            // Datum geändert → Änderungen vorhanden
                                            has_unsaved_changes.set(true);
                                        }
                                        Err(e) => {
                                            console::error_1(
                                                &format!("Date format not correct: {}", e).into(),
                                            );
                                        }
                                    }
                                },
                            }
                        }
                    }

                    InputError { error: error_signal.read() }
                }

                // Card footer – action buttons
                div { class: "px-5 pb-5 pt-1 flex flex-wrap items-center gap-3",

                    // Save-Button: disabled wenn keine ungespeicherten Änderungen
                    div { class: "{save_btn_wrapper_class}",
                        ConfirmButton {
                            action: on_save,
                            text: "Save".to_string(),
                            error_signal: error_name_signal.clone(),
                        }
                    }

                    if is_cloud {
                        SecondaryButton {
                            action: async_action!(
                                {
                                    match download_cook_and_run(cook_and_run_meta.id).await {
                                        Ok(cook_and_run_id) => {
                                            navigator.clone().push(Route::Overview { cook_and_run_id });
                                        }
                                        Err(e) => {
                                            console::error_1(
                                                &format!("Error downloading project: {}", e).into(),
                                            );
                                        }
                                    }
                                }
                            ),
                            text: "Download".to_string(),
                            error_signal: error_login_signal.clone(),
                        }
                    } else {
                        SecondaryButton {
                            action: async_action!(
                                {
                                    match upload_cook_and_run(cook_and_run_meta.id).await {
                                        Ok(cook_and_run_id) => {
                                            console::log_1(
                                                &format!("Project successfully uploaded").into(),
                                            );
                                            navigator.clone().push(Route::Overview { cook_and_run_id });
                                        }
                                        Err(e) => {
                                            console::error_1(
                                                &format!("Error uploading project: {}", e).into(),
                                            );
                                        }
                                    }
                                }
                            ),
                            text: "Upload".to_string(),
                            error_signal: error_login_signal.clone(),
                        }
                    }

                    SecondaryButton {
                        action: async_action!(
                            {
                                if let Err(e) = export_file(cook_and_run_meta.id).await {
                                    console::error_1(
                                        &format!("Error exporting file: {}", e).into(),
                                    );
                                }
                            }
                        ),
                        text: "Export".to_string(),
                    }

                    div { class: "ml-auto",
                        WarnButton {
                            action: async_action!({ delete_dialog_signal.set(true); }),
                            text: "Delete Project".to_string(),
                        }
                    }
                }
            }

            // ── Storage info card ─────────────────────────────────
            if is_cloud {
                StorageInfoCard {
                    title: "Cloud Project",
                    icon_color: "text-amber-500",
                    icon_path: "M16.88 9.94a5 5 0 00-9.72-1.47A4 4 0 006 17h9a4 4 0 001.88-7.06z",
                    lines: vec![
                        "This project is stored in the cloud.".to_string(),
                        "Your data is synced across devices and backed up automatically.".to_string(),
                        "Offline work is not available. Use the Download button to create a local copy.".to_string(),
                    ],
                }
            } else {
                StorageInfoCard {
                    title: "Local Project",
                    icon_color: "text-zinc-400",
                    icon_path: "M3 5a2 2 0 012-2h10a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2V5zm2 0v8h10V5H5zm4 10h2v2H9v-2z",
                    lines: vec![
                        "This project is stored only on this device.".to_string(),
                        "Upload it to the cloud to enable syncing, backups, and collaboration.".to_string(),
                        "Cloud features require you to be logged in.".to_string(),
                    ],
                }
            }
        }

        // ── Delete dialog ─────────────────────────────────────────
        if *delete_dialog_signal.read() {
            DeleteProjectDialog {
                delete_project_signal: delete_dialog_signal.clone(),
                project_id: cook_and_run_meta.id,
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Storage info card
// ─────────────────────────────────────────────

#[component]
fn StorageInfoCard(
    title: &'static str,
    icon_color: &'static str,
    icon_path: &'static str,
    lines: Vec<String>,
) -> Element {
    rsx! {
        div { class: "bg-white rounded-2xl border border-amber-100 shadow-sm overflow-hidden",

            div { class: "px-5 py-3.5 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5",
                svg {
                    class: "w-4 h-4 shrink-0 {icon_color}",
                    fill: "currentColor",
                    view_box: "0 0 20 20",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "{icon_path}" }
                }
                span { class: "text-sm font-semibold text-zinc-800", "{title}" }
            }

            div { class: "px-5 py-4 space-y-1.5",
                for (i, line) in lines.iter().enumerate() {
                    p {
                        class: if i == lines.len() - 1 {
                            "text-sm font-medium text-zinc-700"
                        } else {
                            "text-sm text-zinc-500"
                        },
                        "{line}"
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Delete dialog
// ─────────────────────────────────────────────

#[component]
fn DeleteProjectDialog(delete_project_signal: Signal<bool>, project_id: Uuid) -> Element {
    let mut delete_loading_signal = use_signal(|| false);
    let navigator = use_navigator();
    rsx! {
        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen \
                      justify-center items-center bg-black/20 z-50",
            div { class: "relative bg-white rounded-2xl border border-red-100 \
                          shadow-xl w-96 overflow-hidden",

                // Dialog header
                div { class: "px-5 py-4 bg-red-50/70 border-b border-red-100 flex items-center gap-2.5",
                    div { class: "w-1.5 h-5 rounded-full bg-red-400/70" }
                    span { class: "text-base font-semibold text-red-700", "Delete Project" }
                }

                // Close button
                CloseButton {
                    onclick: move |_| { delete_project_signal.set(false); },
                }

                // Body
                div { class: "px-5 py-5 space-y-5",
                    p { class: "text-sm text-zinc-600 leading-relaxed",
                        "Deleting this project will "
                        span { class: "font-semibold text-red-600", "permanently" }
                        " remove all data. This action "
                        span { class: "font-semibold text-red-600", "cannot be undone." }
                    }

                    div { class: "flex justify-end",
                        WarnButton {
                            text: if *delete_loading_signal.read() {
                                "Deleting…".to_string()
                            } else {
                                "Delete Project".to_string()
                            },
                            action: async_action!(
                                {
                                    delete_loading_signal.set(true);
                                    let result = delete_cook_and_run_project(project_id).await;
                                    delete_loading_signal.set(false);
                                    if let Err(e) = result {
                                        console::error_1(
                                            &format!("Error deleting project: {}", e).into(),
                                        );
                                    } else {
                                        navigator.clone().push(Route::Dashboard {});
                                    }
                                }
                            ) as AsyncAction,
                        }
                    }
                }
            }
        }
    }
}
