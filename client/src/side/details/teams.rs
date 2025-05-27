use std::sync::{Arc, Mutex};

use dioxus::html::button::value;
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::storage::{ContactData, LocalStorage};

use crate::{
    side::{BlueButton, CloseButton, GreenButton, Input, InputError, RedButton},
    storage::StorageW,
    Route,
};

fn add_team(
    id: Uuid,
    team_name: String,
    address: String,
    allergies: Vec<String>,
) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let team = ContactData {
        id: Uuid::new_v4(),
        team_name,
        address,
        allergies,
        latitude: 0.0,  // Placeholder, should be set based on actual data
        longitude: 0.0, // Placeholder, should be set based on actual data
    };
    let result = storage.add_team_to_cook_and_run(id, team);
    result
}

pub(crate) struct TeamsProps {
    pub project_id: Uuid,
    pub team_list: Vec<TeamProps>,
}

#[component]
pub(crate) fn Teams(props: &TeamsProps) -> Element {
    let mut add_team_dialog_signal: Signal<Element> = use_signal(|| rsx!());
    let add_team_dialog = rsx! {
        AddTeamDialog {
            project_id: props.project_id,
            add_team_dialog_signal: add_team_dialog_signal.clone(),
        }
    };
    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-4", "Teams" }

            GreenButton {
                text: "Create team".to_string(),
                onclick: move |_| {
                    add_team_dialog_signal.set(add_team_dialog.clone());
                },
            }
            GreenButton { text: "Import team".to_string(), onclick: move |_| {} }
            BlueButton { text: "Share team link".to_string(), onclick: move |_| {} }


            // Scrollable grid
            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",
                //add here
                {props.team_list.iter().map(|team| { Team(team) })}
            }
        }
        {add_team_dialog_signal}
    }
}

pub(crate) struct TeamProps {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub allergies: Vec<String>,
}

#[component]
fn Team(props: &TeamProps) -> Element {
    rsx! {
        div { key: {props.id}, class: "border rounded bg-white p-4 shadow",
            h3 { class: "text-lg font-semibold mb-1", "{props.name}" }
            p { class: "text-sm text-gray-600 mb-1", "📍 {props.address}" }
            if !props.allergies.is_empty() {
                p { class: "text-sm text-red-600", "⚠ Allergies: {props.allergies.join(\", \")}" }
            }
        }
    }
}

#[component]
fn AddTeamDialog(add_team_dialog_signal: Signal<Element>, project_id: Uuid) -> Element {
    let mut team_name_signal = use_signal(|| "".to_string());
    let mut team_name_error_signal = use_signal(|| "Team name cannot be empty!".to_string());
    let mut address_signal = use_signal(|| "".to_string());
    let mut address_error_signal = use_signal(|| "Address cannot be empty!".to_string());
    let mut allergies_signal = use_signal(|| "".to_string());

    let mut error_signal = use_signal(|| "-".to_string());
    rsx! {
        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 hover:shadow-lg transition-all cursor-pointer ",

                // Title
                h2 { class: "text-2xl font-semibold text-black-600 mb-4", "Add team" }

                // Input fields
                Input {
                    place_holer: Some("Team name".to_string()),
                    value: team_name_signal.clone(),
                    error_signal: team_name_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let team_name = e.value().trim().to_string();
                        team_name_signal.set(team_name.clone());
                        if team_name.is_empty() {
                            team_name_error_signal.set("Team name cannot be empty!".to_string());
                            error_signal.set("-".to_string());
                        } else {
                            team_name_error_signal.set("".to_string());
                            if address_error_signal.read().is_empty() {
                                error_signal.set("".to_string());
                            }
                        }
                    },
                }
                InputError { error_signal: team_name_error_signal.clone() }
                Input {
                    place_holer: Some("Team address".to_string()),
                    value: address_signal.clone(),
                    error_signal: address_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let address = e.value();
                        address_signal.set(address.clone());
                        if address.is_empty() {
                            address_error_signal.set("Address cannot be empty!".to_string());
                            error_signal.set("-".to_string());
                        } else {
                            address_error_signal.set("".to_string());
                            if team_name_error_signal.read().is_empty() {
                                error_signal.set("".to_string());
                            }
                        }
                    },
                }
                InputError { error_signal: address_error_signal.clone() }
                Input {
                    place_holer: Some("Allergies (comma separated)".to_string()),
                    value: allergies_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let allergies = e.value();
                        allergies_signal.set(allergies.clone());
                    },
                }

                // Close button
                CloseButton {
                    onclick: move |_| {
                        add_team_dialog_signal.set(rsx! {});
                    },
                }

                // Create team button
                GreenButton {
                    text: "Create team".to_string(),
                    error_signal: error_signal.clone(),
                    onclick: move |_| {
                        let result = add_team(
                            project_id,
                            team_name_signal.read().to_string(),
                            address_signal.read().to_string(),
                            allergies_signal.read().split(',').map(|s| s.trim().to_string()).collect(),
                        );
                        if result.is_err() {
                            console::error_1(
                                &format!(
                                    "Error deleting project: {}",
                                    result.err().expect("Expected error"),
                                )
                                    .into(),
                            );
                        } else {
                            add_team_dialog_signal.set(rsx! {});
                        }
                    },
                }
            }
        }
    }
}
