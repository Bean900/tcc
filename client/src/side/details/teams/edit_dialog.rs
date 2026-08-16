//! Dialog zum Bearbeiten eines bestehenden Teams. Enthält zwei Tabs
//! ("Team Data" via `TeamDialog`, "Notes" via `TeamNotes`) sowie den
//! Lösch-Button und das "Save"-Glow-Feedback nach erfolgreichem Update.

use crate::side::details::address::AddressParam;
use crate::storage::{StorageManager, TeamData};
use crate::ui::buttons::{ConfirmButton, DeleteButtonProps};
use crate::ui::dialogs::Modal;
use crate::ui::forms::Checkbox;
use crate::async_action;
use async_std::task::sleep;
use std::time::Duration;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use super::actions::{delete_team, update_team};
use super::form::TeamDialog;
use super::model::{tab_cls, PopUpWindow};
use super::notes::TeamNotes;
use super::validation::check_all;

use crate::ui::buttons::AsyncAction;

#[component]
pub(super) fn EditTeamDialog(
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    team_data: TeamData,
    on_teams_changed: EventHandler<()>,
) -> Element {
    let storage_signal = use_context::<Signal<StorageManager>>();
    let team_id = team_data.id;

    let team_name_signal = use_signal(|| team_data.name.clone());
    let team_name_error_signal = use_signal(String::new);
    let team_email_signal = use_signal(|| team_data.mail.clone().unwrap_or_default());
    let team_email_error_signal = use_signal(String::new);
    let team_tel_signal = use_signal(|| team_data.phone.clone().unwrap_or_default());
    let team_tel_error_signal = use_signal(String::new);
    let members_signal = use_signal(|| {
        team_data
            .members
            .map_or_else(String::new, |v| v.to_string())
    });
    let members_error_signal = use_signal(String::new);
    let diets_signal = use_signal(|| team_data.diets.clone().unwrap_or_default());
    let mut needs_check_signal = use_signal(|| team_data.needs_check);
    let error_signal = use_signal(String::new);
    let mut is_edit_team_signal = use_signal(|| true);
    let address_param = AddressParam::new(&team_data.address);

    let mut save_success_signal = use_signal(|| false);
    let mut has_unsaved_changes = use_signal(|| false);

    // Der frühere `use_effect`, der bei jeder Feldänderung das komplette
    // Formular erneut validiert hat, wurde entfernt (Phase 2, Punkt 2).
    // Validierung erfolgt jetzt konsistent zu AddTeamDialog:
    // gezielt pro Feld über die oninput-Handler in TeamDialog,
    // vollständig beim Absenden über check_all.

    let delete_button = DeleteButtonProps::new(
        async_action!({
            if let Err(e) = delete_team(storage_signal, project_id, team_id).await {
                console::error_1(&format!("Error deleting team: {e}").into());
            } else {
                on_teams_changed.call(());
                team_dialog_signal.set(PopUpWindow::None);
            }
        }),
        None,
    );

    let is_success = *save_success_signal.read();
    let can_update = *has_unsaved_changes.read();
    let update_btn_wrapper_class = if can_update {
        ""
    } else {
        "opacity-40 pointer-events-none cursor-not-allowed"
    };

    rsx! {
        Modal {
            title: "Edit Team".to_string(),
            on_close: move |_| team_dialog_signal.set(PopUpWindow::None),
            accent: true,
            scrollable: true,
            close_on_backdrop_click: false,
            glow: is_success,
            max_width: Some("max-w-4xl".to_string()),
            children: rsx! {
                    div { class: "flex items-center justify-between border-b border-amber-100 mb-5",
                        div { class: "flex",
                            button {
                                r#type: "button",
                                class: tab_cls(*is_edit_team_signal.read()),
                                onclick: move |_| {
                                    is_edit_team_signal.set(true);
                                },
                                "Team Data"
                            }
                            button {
                                r#type: "button",
                                class: tab_cls(!*is_edit_team_signal.read()),
                                onclick: move |_| {
                                    is_edit_team_signal.set(false);
                                },
                                "Notes"
                            }
                        }
                        div { class: "flex items-center gap-3 pb-1",
                            Checkbox {
                                label: "Needs review".to_string(),
                                checked: *needs_check_signal.read(),
                                onclick: move |_| {
                                    let new_value = !*needs_check_signal.read();
                                    needs_check_signal.set(new_value);
                                    has_unsaved_changes.set(true);
                                },
                            }
                            {delete_button}
                        }
                    }

                    if *is_edit_team_signal.read() {
                        TeamDialog {
                            team_dialog_signal,
                            project_id,
                            team_name_signal,
                            team_name_error_signal,
                            team_email_signal,
                            team_email_error_signal,
                            team_tel_signal,
                            team_tel_error_signal,
                            members_signal,
                            members_error_signal,
                            diets_signal,
                            address_param: address_param.clone(),
                            on_change: move |_| {
                                has_unsaved_changes.set(true);
                            },
                        }

                        div { class: "flex justify-center mt-5",
                            div { class: "{update_btn_wrapper_class}",
                                ConfirmButton {
                                    text: "Update Team".to_string(),
                                    // Signal<String> ist Copy – .clone() war hier unnötig.
                                    error_signal,
                                    action: async_action!(
                                        {
                                            if !check_all(
                                                team_name_signal,
                                                team_name_error_signal,
                                                team_email_signal,
                                                team_email_error_signal,
                                                team_tel_signal,
                                                team_tel_error_signal,
                                                members_error_signal,
                                                members_signal,
                                                address_param.clone(),
                                            ) {
                                                return;
                                            }
                                            let address_data = address_param
                                                .get_address_data()
                                                .expect("Expect no errors when getting address_data!");
                                            let result = update_team(
                                                storage_signal,
                                                project_id,
                                                team_id,
                                                team_name_signal.read().clone(),
                                                diets_signal.read().clone(),
                                                team_email_signal.read().clone(),
                                                team_tel_signal.read().clone(),
                                                members_signal.read().clone(),
                                                address_data,
                                                *needs_check_signal.read(),
                                            ).await;
                                            if let Err(e) = result {
                                                console::error_1(&format!("Error updating team: {e}").into());
                                            } else {
                                                on_teams_changed.call(());
                                                save_success_signal.set(true);
                                                has_unsaved_changes.set(false);
                                                spawn(async move {
                                                    sleep(Duration::from_millis(2000)).await;
                                                    // Guard gegen die in Phase 1 beschriebene Race
                                                    // Condition: Nur schließen, wenn der Dialog
                                                    // währenddessen nicht bereits gewechselt wurde.
                                                    let still_this_dialog = matches!(
                                                        &*team_dialog_signal.read(),
                                                        PopUpWindow::EditTeam(id) if *id == team_id
                                                    );
                                                    if still_this_dialog {
                                                        team_dialog_signal.set(PopUpWindow::None);
                                                    }
                                                });
                                            }
                                        }
                                    ),
                                }
                            }
                        }
                    } else {
                        TeamNotes {
                            project_id,
                            team_id,
                            note_data_list: team_data.note_list,
                            on_teams_changed,
                        }
                    }
            },
        }
    }
}
