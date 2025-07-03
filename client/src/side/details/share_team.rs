use std::sync::{Arc, Mutex};
use std::vec;

use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::side::details::address::{Address, AddressParam};
use crate::side::InputPhoneNumber;
use crate::storage::{AddressData, ContactData, LocalStorage};

use crate::{
    side::{ConfirmButton, Input, InputError, InputNumber},
    storage::StorageW,
};

fn add_team(
    id: Uuid,
    team_name: String,
    diets: Vec<String>,
    mail: String,
    tel: String,
    members: u32,
    address_data: AddressData,
) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");

    let team = ContactData {
        id: Uuid::new_v4(),
        team_name,
        address: address_data,
        diets,
        mail,
        phone_number: tel,
        members,
        needs_check: true,
        notes: vec![],
    };
    let result = storage.add_team_to_cook_and_run(id, team);
    result
}

#[component]
pub fn ShareTeam(cook_and_run_id: Uuid, share_id: Uuid) -> Element {
    rsx! {
        AddTeamDialog { project_id: cook_and_run_id, share_id }
    }
}

#[component]
fn AddTeamDialog(project_id: Uuid, share_id: Uuid) -> Element {
    let team_name_signal = use_signal(|| "".to_string());
    let team_name_error_signal = use_signal(|| "".to_string());

    let contact_email_signal = use_signal(|| "".to_string());
    let contact_email_error_signal = use_signal(|| "".to_string());

    let contact_tel_signal = use_signal(|| "".to_string());
    let contact_tel_error_signal = use_signal(|| "".to_string());

    let members_signal = use_signal(|| "".to_string());
    let members_error_signal = use_signal(|| "".to_string());

    let diets_signal = use_signal(|| "".to_string());

    let address_param = AddressParam::default();

    let shared = use_signal(|| false);
    if *shared.read() {
        rsx! {}
    } else {
        rsx! {

            div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
                div { class: "relative bg-white shadow-md rounded-xl p-6 hover:shadow-lg transition-all cursor-pointer w-224",
                    // Title
                    h2 { class: "text-2xl font-semibold text-black-600 mb-4", "Add Team" }
                    TeamDialog {
                        project_id,
                        team_name_signal,
                        team_name_error_signal,
                        contact_email_signal,
                        contact_email_error_signal,
                        contact_tel_signal,
                        contact_tel_error_signal,
                        members_signal,
                        members_error_signal,
                        diets_signal,
                        address_param: address_param.clone(),
                    }


                    // Create team button
                    div { class: "flex justify-center mt-4",
                        ConfirmButton {
                            text: "Create Team".to_string(),
                            onclick: move |_| {
                                if !check_all(
                                    team_name_signal,
                                    team_name_error_signal,
                                    contact_email_signal,
                                    contact_email_error_signal,
                                    contact_tel_signal,
                                    contact_tel_error_signal,
                                    members_error_signal,
                                    members_signal,
                                    address_param.clone(),
                                ) {
                                    return;
                                }
                                let result = add_team(
                                    project_id,
                                    team_name_signal.read().trim().to_string(),
                                    diets_signal.read().split(',').map(|s| s.trim().to_string()).collect(),
                                    contact_email_signal.read().trim().to_string(),
                                    contact_tel_signal.read().trim().to_string(),
                                    members_signal.read().parse::<u32>().unwrap_or(0),
                                    address_param
                                        .get_address_data()
                                        .expect("Expext no errors when getting address_data!"),
                                );
                                if result.is_err() {
                                    console::error_1(
                                        &format!(
                                            "Error creating team: {}",
                                            result.err().expect("Expected error"),
                                        )
                                            .into(),
                                    );
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TeamDialog(
    project_id: Uuid,
    team_name_signal: Signal<String>,
    team_name_error_signal: Signal<String>,
    contact_email_signal: Signal<String>,
    contact_email_error_signal: Signal<String>,
    contact_tel_signal: Signal<String>,
    contact_tel_error_signal: Signal<String>,
    members_signal: Signal<String>,
    members_error_signal: Signal<String>,
    diets_signal: Signal<String>,
    address_param: AddressParam,
) -> Element {
    rsx! {
        div { class: "flex flex-col md:flex-row",
            // Left side: Team details
            div { class: "flex-1 pr-4 border-r border-gray-300", // Team name
                label { class: "block font-semibold text-gray-700 mb-1", "Team Name" }
                Input {
                    place_holer: Some("e.g. The Chili Chasers".to_string()),
                    is_error: !team_name_error_signal.read().is_empty(),
                    value: team_name_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let team_name = e.value();
                        team_name_signal.set(team_name.clone());
                        check_team_name(team_name_signal, team_name_error_signal);
                    },
                }
                InputError { error: team_name_error_signal.read() }

                // Contact E-Mail
                label { class: "block font-semibold text-gray-700 mb-1", "Contact E-Mail" }
                Input {
                    place_holer: Some("e.g. chili@chasers.de".to_string()),
                    is_error: !contact_email_error_signal.read().is_empty(),
                    value: contact_email_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let contact_email = e.value();
                        contact_email_signal.set(contact_email.clone());
                        check_contact_email(contact_email_signal, contact_email_error_signal);
                    },
                }
                InputError { error: contact_email_error_signal.read() }

                // Contact Phone Number
                label { class: "block font-semibold text-gray-700 mb-1", "Contact Phone Number" }
                InputPhoneNumber {
                    place_holer: Some("e.g. +49 1234 56789".to_string()),
                    is_error: !contact_tel_error_signal.read().is_empty(),
                    value: contact_tel_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let contact_tel = e.value();
                        contact_tel_signal.set(contact_tel.clone());
                        check_contact_tel(contact_tel_signal, contact_tel_error_signal);
                    },
                }
                InputError { error: contact_tel_error_signal.read() }

                // Number of Members
                label { class: "block font-semibold text-gray-700 mb-1", "Number of Members" }
                InputNumber {
                    place_holer: Some("e.g. 2".to_string()),
                    value: members_signal.clone(),
                    is_error: !members_error_signal.read().is_empty(),
                    oninput: move |e: Event<FormData>| {
                        let members = e.value();
                        members_signal.set(members.clone());
                        check_members(members_signal, members_error_signal);
                    },
                }
                InputError { error: members_error_signal.read() }

                // Diets
                label { class: "block font-semibold text-gray-700 mb-1", "Dietary requirements" }
                div { class: "w-full",
                    Input {
                        place_holer: Some("e.g. vegetarian, nut allergy, halal ...".to_string()),
                        is_error: false,
                        value: diets_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let diets = e.value();
                            diets_signal.set(diets.clone());
                        },
                    }
                }

            }

            // Right side: Address block
            div { class: "flex-1 pl-4",
                Address { param: address_param }
            }
        }
    }
}

fn check_all(
    team_name_signal: Signal<String>,
    team_name_error_signal: Signal<String>,
    contact_email_signal: Signal<String>,
    contact_email_error_signal: Signal<String>,
    contact_tel_signal: Signal<String>,
    contact_tel_error_signal: Signal<String>,
    members_error_signal: Signal<String>,
    members_signal: Signal<String>,
    address_param: AddressParam,
) -> bool {
    let team_name_check = check_team_name(team_name_signal, team_name_error_signal);
    let contact_email_check = check_contact_email(contact_email_signal, contact_email_error_signal);
    let contact_tel_check = check_contact_tel(contact_tel_signal, contact_tel_error_signal);
    let member_check = check_members(members_signal, members_error_signal);
    let address_check = address_param.check_address_data().is_ok();
    team_name_check && contact_email_check && contact_tel_check && member_check && address_check
}

fn check_team_name(
    team_name_signal: Signal<String>,
    mut team_name_error_signal: Signal<String>,
) -> bool {
    let team_name = team_name_signal.read();
    if team_name.is_empty() {
        team_name_error_signal.set("Team name cannot be empty!".to_string());
        false
    } else {
        team_name_error_signal.set("".to_string());
        true
    }
}

fn check_contact_email(
    contact_email_signal: Signal<String>,
    mut contact_email_error_signal: Signal<String>,
) -> bool {
    let contact_email = contact_email_signal.read();
    if contact_email.is_empty() {
        contact_email_error_signal.set("Contact E-Mail cannot be empty!".to_string());
        false
    } else if !contact_email.contains('@') || !contact_email.contains('.') {
        contact_email_error_signal.set("Please enter a valid email address!".to_string());
        false
    } else {
        contact_email_error_signal.set("".to_string());
        true
    }
}

fn check_contact_tel(
    contact_tel_signal: Signal<String>,
    mut contact_tel_error_signal: Signal<String>,
) -> bool {
    let contact_tel = contact_tel_signal.read();
    if contact_tel.is_empty() {
        contact_tel_error_signal.set("Contact phone number cannot be empty!".to_string());
        false
    } else {
        contact_tel_error_signal.set("".to_string());
        true
    }
}

fn check_members(members_signal: Signal<String>, mut members_error_signal: Signal<String>) -> bool {
    let members = members_signal.read();
    if members.is_empty() {
        members_error_signal.set("Number of Members cannot be empty!".to_string());
        false
    } else if members.parse::<u32>().is_err() {
        members_error_signal.set("Please enter a valid number!".to_string());
        false
    } else if members.parse::<u32>().unwrap() == 0 {
        members_error_signal.set("Number of Members must be greater than 0!".to_string());
        false
    } else {
        members_error_signal.set("".to_string());
        true
    }
}
