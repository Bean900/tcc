//! Der "Notes"-Tab im Edit-Dialog: Formular zum Anlegen neuer Notizen
//! plus die Liste der bisherigen Notizen zu einem Team.

use crate::storage::{NoteData, StorageManager};
use crate::ui::buttons::ConfirmButton;
use crate::ui::forms::{Input, InputError, TextArea};
use crate::ui::tokens::LBL;
use crate::async_action;
use chrono::{Local, Utc};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use super::actions::add_team_note;

use crate::ui::buttons::AsyncAction;

#[component]
pub(super) fn TeamNotes(
    project_id: Uuid,
    team_id: Uuid,
    note_data_list: Vec<NoteData>,
    on_teams_changed: EventHandler<()>,
) -> Element {
    let storage_signal = use_context::<Signal<StorageManager>>();

    let mut create_note_headline_signal = use_signal(String::new);
    let mut create_note_headline_error_signal = use_signal(String::new);
    let mut create_note_content_signal = use_signal(String::new);
    let mut create_note_content_error_signal = use_signal(String::new);
    let mut create_note_error_signal = use_signal(String::new);
    let mut creating_error_signal = use_signal(String::new);

    let mut sorted_note_list = note_data_list;
    sorted_note_list.sort_by(|a, b| b.created.cmp(&a.created));
    // Hinweis: Dieser use_signal-Init läuft nur beim ersten Mount von
    // TeamNotes. Da EditTeamDialog pro geöffnetem Team eine frische
    // Komponenteninstanz ist (Team-ID steckt im Enum-Discriminant von
    // PopUpWindow), ist das unkritisch. Die lokale Liste dient primär
    // der sofortigen optimistischen UI-Aktualisierung nach dem Anlegen
    // einer Notiz; die eigentliche Quelle der Wahrheit bleibt team_list
    // in Teams, das über on_teams_changed synchron gehalten wird.
    let mut sorted_note_list_signal = use_signal(|| sorted_note_list);

    rsx! {
        div { class: "flex flex-col md:flex-row gap-6",
            div { class: "flex-1 md:border-r border-amber-100 md:pr-5 space-y-3",
                label { class: "{LBL}", "New Note" }

                div {
                    label { class: "{LBL}", "Headline" }
                    Input {
                        place_holer: Some("e.g. Participation fee".to_string()),
                        value: create_note_headline_signal.clone(),
                        is_error: !create_note_headline_error_signal.read().is_empty(),
                        oninput: move |e: Event<FormData>| {
                            let headline = e.value();
                            create_note_headline_signal.set(headline.clone());
                            if headline.is_empty() {
                                create_note_headline_error_signal
                                    .set("Headline cannot be empty!".to_string());
                            } else {
                                create_note_headline_error_signal.set(String::new());
                                if create_note_content_error_signal.read().is_empty() {
                                    create_note_error_signal.set(String::new());
                                }
                            }
                        },
                    }
                    InputError { error: create_note_headline_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Content" }
                    TextArea {
                        placeholder: Some("e.g. Participation fee is partially paid!".to_string()),
                        value: create_note_content_signal.read().clone(),
                        is_error: !create_note_content_error_signal.read().is_empty(),
                        oninput: move |e: Event<FormData>| {
                            let content = e.value();
                            create_note_content_signal.set(content.clone());
                            if content.is_empty() {
                                create_note_content_error_signal
                                    .set("Content cannot be empty!".to_string());
                                create_note_error_signal.set("-".to_string());
                            } else {
                                create_note_content_error_signal.set(String::new());
                                if create_note_headline_error_signal.read().is_empty() {
                                    create_note_error_signal.set(String::new());
                                }
                            }
                        },
                    }
                    InputError { error: create_note_content_error_signal.read() }
                }

                ConfirmButton {
                    text: "Post Note".to_string(),
                    error_signal: create_note_error_signal,
                    action: async_action!(
                        {
                            let headline = create_note_headline_signal.read().trim().to_string();
                            let content = create_note_content_signal.read().trim().to_string();
                            let mut has_error = false;
                            if headline.is_empty() {
                                create_note_headline_error_signal
                                    .set("Headline cannot be empty!".to_string());
                                has_error = true;
                            }
                            if content.is_empty() {
                                create_note_content_error_signal
                                    .set("Content cannot be empty!".to_string());
                                has_error = true;
                            }
                            if has_error {
                                create_note_error_signal.set("-".to_string());
                                return;
                            }
                            let result = add_team_note(
                                storage_signal,
                                project_id,
                                team_id,
                                headline.clone(),
                                content.clone(),
                            ).await;
                            if let Err(e) = result {
                                console::error_1(&format!("Error creating note: {e}").into());
                                creating_error_signal.set("Error creating note!".to_string());
                            } else {
                                let new_note = NoteData {
                                    id: Uuid::new_v4(),
                                    headline,
                                    content,
                                    created: Utc::now(),
                                };
                                // Sofortiges optimistisches UI-Update ...
                                sorted_note_list_signal.write().insert(0, new_note);
                                create_note_headline_signal.set(String::new());
                                create_note_content_signal.set(String::new());
                                create_note_headline_error_signal.set(String::new());
                                create_note_content_error_signal.set(String::new());
                                create_note_error_signal.set(String::new());
                                creating_error_signal.set(String::new());
                                // ... plus explizite Invalidierung der
                                // Source-of-Truth-Resource (Phase 2, Punkt 5),
                                // statt uns auf implizites Storage-Signal-
                                // Tracking zu verlassen.
                                on_teams_changed.call(());
                            }
                        }
                    ),
                }
                InputError { error: creating_error_signal.read() }
            }

            div { class: "flex-1",
                label { class: "{LBL}", "Notes" }
                div { class: "space-y-2 overflow-y-auto max-h-80 mt-1",
                    for note_data in sorted_note_list_signal.read().iter() {
                        Note {
                            key: "{note_data.id}",
                            note_data: note_data.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Note(note_data: NoteData) -> Element {
    let created = note_data.created
        .with_timezone(&Local)
        .format("%d.%m.%Y, %H:%M Uhr")
        .to_string();

    rsx! {
        div { class: "bg-amber-50/50 rounded-xl border border-amber-100 px-4 py-3",
            div { class: "flex justify-between items-baseline gap-2 mb-1",
                span { class: "text-sm font-semibold text-zinc-800 truncate", "{note_data.headline}" }
                span { class: "text-[11px] text-zinc-400 shrink-0", "{created}" }
            }
            p { class: "text-xs text-zinc-600 leading-relaxed", "{note_data.content}" }
        }
    }
}
