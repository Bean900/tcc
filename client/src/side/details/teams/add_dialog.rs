//! Dialog zum Anlegen eines neuen Teams. Bettet das gemeinsame
//! `TeamDialog`-Formular ein und ruft beim Bestätigen `actions::add_team`
//! auf.

use crate::side::details::address::AddressParam;
use crate::storage::StorageManager;
use crate::ui::buttons::ConfirmButton;
use crate::ui::dialogs::Modal;
use crate::async_action;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use super::actions::add_team;
use super::form::TeamDialog;
use super::model::PopUpWindow;
use super::validation::check_all;

use crate::ui::buttons::AsyncAction;

#[component]
pub(super) fn AddTeamDialog(
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    on_teams_changed: EventHandler<()>,
) -> Element {
    let storage_signal = use_context::<Signal<StorageManager>>();

    let team_name_signal = use_signal(String::new);
    let team_name_error_signal = use_signal(String::new);
    let team_email_signal = use_signal(String::new);
    let team_email_error_signal = use_signal(String::new);
    let team_tel_signal = use_signal(String::new);
    let team_tel_error_signal = use_signal(String::new);
    let members_signal = use_signal(String::new);
    let members_error_signal = use_signal(String::new);
    let diets_signal = use_signal(String::new);
    let address_param = AddressParam::default();

    rsx! {
        Modal {
            title: "Add Team".to_string(),
            on_close: move |_| team_dialog_signal.set(PopUpWindow::None),
            accent: true,
            scrollable: true,
            close_on_backdrop_click: false,
            max_width: Some("max-w-4xl".to_string()),
            children: rsx! {
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
                    on_change: move |_| {},
                }

                div { class: "flex justify-center mt-5",
                    ConfirmButton {
                        text: "Create Team".to_string(),
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
                                let result = add_team(
                                    storage_signal,
                                    project_id,
                                    team_name_signal.read().clone(),
                                    diets_signal.read().clone(),
                                    team_email_signal.read().clone(),
                                    team_tel_signal.read().clone(),
                                    members_signal.read().clone(),
                                    address_data,
                                ).await;
                                if let Err(e) = result {
                                    console::error_1(&format!("Error creating team: {e}").into());
                                } else {
                                    // Explizite Invalidierung statt impliziter
                                    // Storage-Signal-Kopplung (siehe Phase 2, Punkt 5).
                                    on_teams_changed.call(());
                                    team_dialog_signal.set(PopUpWindow::None);
                                }
                            }
                        ),
                    }
                }
            },
        }
    }
}
