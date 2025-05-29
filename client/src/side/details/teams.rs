use std::sync::{Arc, Mutex};
use std::{result, vec};

use chrono::{DateTime, Utc};
use dioxus::html::desc;
use dioxus::prelude::*;
use serde_json::error;
use uuid::Uuid;
use web_sys::console;

use crate::address_connector::get_address;
use crate::storage::{ContactData, LocalStorage, NoteData};

use crate::{
    side::{
        BlueButton, CloseButton, DeleteButton, GreenButton, Input, InputError, InputMultirow,
        InputNumber, RedButton,
    },
    storage::StorageW,
};

fn add_team(
    id: Uuid,
    team_name: String,
    address: String,
    allergies: Vec<String>,
    latitude: f64,
    longitude: f64,
    mail: String,
    members: u32,
) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let team = ContactData {
        id: Uuid::new_v4(),
        team_name,
        address,
        allergies,
        latitude,
        longitude,
        mail,
        members,
        needs_check: false,
        notes: vec![],
    };
    let result = storage.add_team_to_cook_and_run(id, team);
    result
}

fn update_team(id: Uuid, team: ContactData) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let result = storage.update_team_in_cook_and_run(id, team);
    result
}

fn add_team_note(
    id: Uuid,
    team_id: Uuid,
    headline: String,
    description: String,
) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let result = storage.create_team_note_in_cook_and_run(id, team_id, headline, description);
    result
}

fn update_team_needs_check(id: Uuid, team_id: Uuid, needs_check: bool) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let result = storage.update_team_needs_ckeck_in_cook_and_run(id, team_id, needs_check);
    result
}

fn delete_team(id: Uuid, team_id: Uuid) -> Result<(), String> {
    let storage = use_context::<Arc<Mutex<LocalStorage>>>();
    let mut storage = storage.lock().expect("Expected storage lock");
    let result = storage.delete_team_in_cook_and_run(id, team_id);
    result
}

pub(crate) struct TeamsProps {
    pub project_id: Uuid,
    pub team_list: Vec<TeamCardProps>,
}

#[component]
pub(crate) fn Teams(props: &TeamsProps) -> Element {
    let mut team_dialog_signal: Signal<Element> = use_signal(|| rsx!());
    let add_team_dialog = rsx! {
        AddTeamDialog {
            project_id: props.project_id,
            team_dialog_signal: team_dialog_signal.clone(),
        }
    };
    rsx! {
        section {
            h2 { class: "text-2xl font-bold mb-4", "Teams" }

            // Scrollable grid
            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 p-6 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",


                {
                    props
                        .team_list
                        .iter()
                        .map(|team| {
                            let project_id = props.project_id;
                            let team_card_props = team.clone();
                            let background = if team.needs_check {
                                "bg-orange-100"
                            } else {
                                "bg-white"
                            };
                            rsx! {
                                a {
                                    key: {team.id},
                                    onclick: move |_| {
                                        team_dialog_signal.set(rsx! {
                                            EditTeamDialog {
                                                team_dialog_signal: team_dialog_signal.clone(),
                                                project_id,
                                                team_card_props: team_card_props.clone(),
                                            }
                                        });
                                    },
                                
                                
                                    class: "{background} relative  shadow-md rounded-xl p-6  hover:shadow-lg transition-all cursor-pointer hover:scale-105",
                                    {TeamCard(team)}
                                }
                            }
                        })
                }

                a {
                    class: "border-4 border-dashed border-gray-300 rounded-xl p-6   flex items-center justify-center text-gray-400 hover:bg-gray-50 hover:text-blue-500 hover:scale-105 transition-all duration-200 cursor-pointer",
                    onclick: move |_| {
                        team_dialog_signal.set(add_team_dialog.clone());
                    },
                    div {

                        div { class: "text-5xl font-bold", "+" }
                    }
                }
            }
        
        }
        {team_dialog_signal}
    }
}

#[derive(PartialEq, Clone)]
pub(crate) struct TeamCardProps {
    pub id: Uuid,
    pub name: String,
    pub contact_email: String,
    pub members: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub address: String,
    pub allergies: Vec<String>,
    pub needs_check: bool,
    pub notes: Vec<NoteProps>,
}

#[derive(Clone, PartialEq)]
pub struct NoteProps {
    pub id: Uuid,
    pub headline: String,
    pub description: String,
    pub created: String,
}

#[component]
fn TeamCard(props: &TeamCardProps) -> Element {
    rsx! {
        div {
            // Name
            h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "{props.name}" }
            // Address
            p { class: "text-sm text-gray-600 mb-1", "📍 {props.address}" }
            // Needs Check Indicator
            if props.needs_check {
                div { class: "absolute top-2 right-2 bg-red-500 text-white text-xs font-bold rounded-full px-2 py-1",
                    "!"
                }
            }
        }
    }
}

#[component]
fn AddTeamDialog(team_dialog_signal: Signal<Element>, project_id: Uuid) -> Element {
    let team_name_signal = use_signal(|| "".to_string());
    let team_name_error_signal = use_signal(|| "".to_string());

    let contact_email_signal = use_signal(|| "".to_string());
    let contact_email_error_signal = use_signal(|| "".to_string());

    let members_signal = use_signal(|| "".to_string());
    let members_error_signal = use_signal(|| "".to_string());

    let address_signal_addr = use_signal(|| "".to_string());
    let address_signal_lat = use_signal(|| "".to_string());
    let address_signal_lon = use_signal(|| "".to_string());

    let address_error_signal = use_signal(|| "".to_string());

    let allergies_signal = use_signal(|| "".to_string());

    let error_signal = use_signal(|| "".to_string());
    rsx! {

        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 hover:shadow-lg transition-all cursor-pointer w-224",
                // Title
                h2 { class: "text-2xl font-semibold text-black-600 mb-4", "Add Team" }
                TeamDialog {
                    team_dialog_signal,
                    project_id,
                    team_name_signal,
                    team_name_error_signal,
                    contact_email_signal,
                    contact_email_error_signal,
                    members_signal,
                    members_error_signal,
                    address_signal_addr,
                    address_signal_lat,
                    address_signal_lon,
                    address_error_signal,
                    allergies_signal,
                    error_signal,
                }
                // Close button
                CloseButton {
                    onclick: move |_| {
                        team_dialog_signal.set(rsx! {});
                    },
                }

                // Create team button
                div { class: "flex justify-center mt-4",
                    GreenButton {
                        text: "Create Team".to_string(),
                        error_signal: error_signal.clone(),
                        onclick: move |_| {
                            if !check_all(
                                &team_name_signal.read(),
                                &contact_email_signal.read(),
                                &members_signal.read(),
                                &address_signal_addr.read(),
                                &address_signal_lat.read(),
                                &address_signal_lon.read(),
                                team_name_error_signal,
                                contact_email_error_signal,
                                members_error_signal,
                                address_error_signal,
                                error_signal,
                            ) {
                                return;
                            }
                            let result = add_team(
                                project_id,
                                team_name_signal.read().trim().to_string(),
                                address_signal_addr.read().trim().to_string(),
                                allergies_signal.read().split(',').map(|s| s.trim().to_string()).collect(),
                                address_signal_lat.read().parse::<f64>().unwrap_or(0.0),
                                address_signal_lon.read().parse::<f64>().unwrap_or(0.0),
                                contact_email_signal.read().trim().to_string(),
                                members_signal.read().parse::<u32>().unwrap_or(0),
                            );
                            if result.is_err() {
                                console::error_1(
                                    &format!(
                                        "Error creating team: {}",
                                        result.err().expect("Expected error"),
                                    )
                                        .into(),
                                );
                            } else {
                                team_dialog_signal.set(rsx! {});
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn EditTeamDialog(
    team_dialog_signal: Signal<Element>,
    project_id: Uuid,
    team_card_props: TeamCardProps,
) -> Element {
    let team_setting_card_props = team_card_props.clone();

    let team_name_signal = use_signal(|| team_card_props.name.clone());
    let team_name_error_signal = use_signal(|| "".to_string());

    let contact_email_signal = use_signal(|| team_card_props.contact_email.clone());
    let contact_email_error_signal = use_signal(|| "".to_string());

    let members_signal = use_signal(|| team_card_props.members.to_string());
    let members_error_signal = use_signal(|| "".to_string());

    let address_signal_addr = use_signal(|| team_card_props.address.clone());
    let address_signal_lat = use_signal(|| team_card_props.latitude.to_string());
    let address_signal_lon = use_signal(|| team_card_props.longitude.to_string());

    let address_error_signal = use_signal(|| "".to_string());

    let allergies_signal = use_signal(|| team_card_props.allergies.join(", "));

    let needs_check_signal = use_signal(|| team_card_props.needs_check);

    let error_signal = use_signal(|| "".to_string());

    let mut is_edit_team_signal = use_signal(|| true);

    let mut need_check_signal = use_signal(|| team_card_props.needs_check);

    use_effect(move || {
        check_all(
            &team_card_props.name.clone(),
            &team_card_props.contact_email.clone(),
            &team_card_props.members.to_string(),
            &team_card_props.address.clone(),
            &team_card_props.latitude.to_string(),
            &team_card_props.longitude.to_string(),
            team_name_error_signal,
            contact_email_error_signal,
            members_error_signal,
            address_error_signal,
            error_signal,
        );
        is_no_error(
            team_name_error_signal,
            contact_email_error_signal,
            members_error_signal,
            address_error_signal,
            error_signal.clone(),
        );
    });

    rsx! {

        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 hover:shadow-lg transition-all cursor-pointer w-224",
                // Title
                h2 { class: "text-2xl font-semibold text-black-600 mb-4", "Edit Team" }


                div { class: "flex border-b border-gray-300 mb-4 items-center justify-between",
                    div { class: "flex",
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                is_edit_team_signal.set(true);
                            },
                            class: if *is_edit_team_signal.read() { "px-4 py-2 font-semibold text-sm text-blue-600 border-b-2 border-blue-600" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-blue-600" },
                            "Team Data"
                        }
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                is_edit_team_signal.set(false);
                            },
                            class: if !*is_edit_team_signal.read() { "px-4 py-2 font-semibold text-sm text-blue-600 border-b-2 border-blue-600" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-blue-600" },
                            "Notes"
                        }
                    }
                    div {
                        div { class: "flex items-center space-x-2",
                            label { class: "text-sm text-gray-600", "Team needs check:" }
                            input {
                                r#type: "checkbox",
                                checked: need_check_signal,
                                class: "text-blue-600 rounded",
                                onclick: move |_| {
                                    let new_value = !*need_check_signal.read();
                                    let result = update_team_needs_check(project_id, team_card_props.id, new_value);
                                    if result.is_err() {
                                        console::error_1(
                                            &format!(
                                                "Error updating team needs check: {}",
                                                result.err().expect("Expected error"),
                                            )
                                                .into(),
                                        );
                                        need_check_signal.set(!new_value);
                                    } else {
                                        need_check_signal.set(new_value);
                                        team_card_props.needs_check = new_value;
                                    }
                                },
                            }
                        }
                    }
                    DeleteButton {
                        onclick: move |_| {
                            let result = delete_team(project_id, team_card_props.id);
                            if result.is_err() {
                                console::error_1(
                                    &format!(
                                        "Error deleting team: {}",
                                        result.err().expect("Expected error"),
                                    )
                                        .into(),
                                );
                            } else {
                                team_dialog_signal.set(rsx! {});
                            }
                        },
                    }
                }

                // Close button
                CloseButton {
                    onclick: move |_| {
                        team_dialog_signal.set(rsx! {});
                    },
                }

                if *is_edit_team_signal.read() {

                    TeamDialog {
                        team_dialog_signal,
                        project_id,
                        team_name_signal,
                        team_name_error_signal,
                        contact_email_signal,
                        contact_email_error_signal,
                        members_signal,
                        members_error_signal,
                        address_signal_addr,
                        address_signal_lat,
                        address_signal_lon,
                        address_error_signal,
                        allergies_signal,
                        error_signal,
                    }


                    // Create team button
                    div { class: "flex justify-center mt-4",
                        GreenButton {
                            text: "Update Team".to_string(),
                            error_signal: error_signal.clone(),
                            onclick: move |_| {
                                if !check_all(
                                    &team_name_signal.read(),
                                    &contact_email_signal.read(),
                                    &members_signal.read(),
                                    &address_signal_addr.read(),
                                    &address_signal_lat.read(),
                                    &address_signal_lon.read(),
                                    team_name_error_signal,
                                    contact_email_error_signal,
                                    members_error_signal,
                                    address_error_signal,
                                    error_signal,
                                ) {
                                    return;
                                }
                                let result = update_team(
                                    project_id,
                                    ContactData {
                                        id: team_card_props.id,
                                        team_name: team_name_signal.read().trim().to_string(),
                                        address: address_signal_addr.read().trim().to_string(),
                                        latitude: address_signal_lat.read().parse::<f64>().unwrap_or(0.0),
                                        longitude: address_signal_lon.read().parse::<f64>().unwrap_or(0.0),
                                        mail: contact_email_signal.read().trim().to_string(),
                                        members: members_signal.read().parse::<u32>().unwrap_or(0),
                                        allergies: allergies_signal
                                            .read()
                                            .split(',')
                                            .map(|s| s.trim().to_string())
                                            .collect(),
                                        needs_check: *needs_check_signal.read(),
                                        notes: vec![],
                                    },
                                );
                                if result.is_err() {
                                    console::error_1(
                                        &format!(
                                            "Error updating team: {}",
                                            result.err().expect("Expected error"),
                                        )
                                            .into(),
                                    );
                                } else {
                                    team_dialog_signal.set(rsx! {});
                                }
                            },
                        }
                    }
                } else {
                    TeamNotes { project_id, props: team_setting_card_props }
                }
            }
        }
    }
}

#[component]
fn TeamDialog(
    team_dialog_signal: Signal<Element>,
    project_id: Uuid,
    team_name_signal: Signal<String>,
    team_name_error_signal: Signal<String>,
    contact_email_signal: Signal<String>,
    contact_email_error_signal: Signal<String>,
    members_signal: Signal<String>,
    members_error_signal: Signal<String>,
    address_signal_addr: Signal<String>,
    address_signal_lat: Signal<String>,
    address_signal_lon: Signal<String>,
    address_error_signal: Signal<String>,
    allergies_signal: Signal<String>,
    error_signal: Signal<String>,
) -> Element {
    let mut address_search_signal = use_signal(|| "".to_string());

    let mut is_auto_address_input_signa = use_signal(|| true);
    rsx! {
        div { class: "flex flex-col md:flex-row",
            // Left side: Team details
            div { class: "flex-1 pr-4 border-r border-gray-300", // Team name
                label { class: "block font-semibold text-gray-700 mb-1", "Team Name" }
                Input {
                    place_holer: Some("e.g. The Chili Chasers".to_string()),
                    value: team_name_signal.clone(),
                    error_signal: team_name_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let team_name = e.value();
                        team_name_signal.set(team_name.clone());
                        check_team_name(&team_name, team_name_error_signal, error_signal);
                        is_no_error(
                            team_name_error_signal,
                            contact_email_error_signal,
                            members_error_signal,
                            address_error_signal,
                            error_signal.clone(),
                        );
                    },
                }
                InputError { error_signal: team_name_error_signal.clone() }

                // Contact E-Mail
                label { class: "block font-semibold text-gray-700 mb-1", "Contact E-Mail" }
                Input {
                    place_holer: Some("e.g. chili@chasers.de".to_string()),
                    value: contact_email_signal.clone(),
                    error_signal: contact_email_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let contact_email = e.value();
                        contact_email_signal.set(contact_email.clone());
                        check_contact_email(&contact_email, contact_email_error_signal, error_signal);
                        is_no_error(
                            team_name_error_signal,
                            contact_email_error_signal,
                            members_error_signal,
                            address_error_signal,
                            error_signal.clone(),
                        );
                    },
                }
                InputError { error_signal: contact_email_error_signal.clone() }

                // Number of Members
                label { class: "block font-semibold text-gray-700 mb-1", "Number of Members" }
                InputNumber {
                    place_holer: Some("e.g. 2".to_string()),
                    value: members_signal.clone(),
                    error_signal: members_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let members = e.value();
                        members_signal.set(members.clone());
                        check_members(&members, members_error_signal, error_signal);
                        is_no_error(
                            team_name_error_signal,
                            contact_email_error_signal,
                            members_error_signal,
                            address_error_signal,
                            error_signal.clone(),
                        );
                    },
                }
                InputError { error_signal: members_error_signal.clone() }

                // Allergies
                label { class: "block font-semibold text-gray-700 mb-1", "Allergies" }
                div { class: "w-full",
                    Input {
                        place_holer: Some("e.g. nuts, gluten ...".to_string()),
                        value: allergies_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let allergies = e.value();
                            allergies_signal.set(allergies.clone());
                        },
                    }
                }
            
            }

            // Right side: Address block
            div { class: "flex-1 pl-4",
                label { class: "block font-semibold text-gray-700 mb-2", "Address" }

                div { class: "flex border-b border-gray-300 mb-4",
                    button {
                        r#type: "button",
                        onclick: move |_| {
                            is_auto_address_input_signa.set(true);
                        },
                        id: "tab-search",
                        class: if *is_auto_address_input_signa.read() { "px-4 py-2 font-semibold text-sm text-blue-600 border-b-2 border-blue-600" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-blue-600" },
                        "Automatic"
                    }
                    button {
                        r#type: "button",
                        onclick: move |_| {
                            is_auto_address_input_signa.set(false);
                        },
                        id: "tab-coords",
                        class: if !*is_auto_address_input_signa.read() { "px-4 py-2 font-semibold text-sm text-blue-600 border-b-2 border-blue-600" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-blue-600" },
                        "Manual"
                    }
                }

                if *is_auto_address_input_signa.read() {
                    // Search Address
                    div { id: "address-search",
                        div { class: "flex items-center justify-between mb-2",
                            label { class: "block font-semibold text-gray-700", "Search Address" }
                            div { class: "flex items-center text-sm text-gray-600",
                                svg {
                                    class: "w-4 h-4 mr-1 text-blue-500",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M13 16h-1v-4h-1m1-4h.01M12 18a6 6 0 100-12 6 6 0 000 12z",
                                    }
                                }
                                span { class: "relative group cursor-pointer",
                                    "Info"
                                    span { class: "absolute bottom-full left-1/2 -translate-x-1/2 mb-1 hidden group-hover:block bg-gray-700 text-white text-xs rounded px-2 py-1 w-max max-w-xs z-10 shadow-md",
                                        "The entered address will be forwarded to Nominatim (OpenStreetMap) for location determination."
                                    }
                                }
                            }
                        }

                        Input {
                            place_holer: Some("Street, City, ZIP code".to_string()),
                            value: address_search_signal.clone(),
                            error_signal: address_error_signal.clone(),
                            oninput: move |e: Event<FormData>| {
                                let address = e.value();
                                address_search_signal.set(address.clone());
                                if address.is_empty() {
                                    address_search_signal.set("Address cannot be empty!".to_string());
                                    error_signal.set("-".to_string());
                                } else {
                                    address_error_signal.set("".to_string());
                                    is_no_error(
                                        team_name_error_signal,
                                        contact_email_error_signal,
                                        members_error_signal,
                                        address_error_signal,
                                        error_signal.clone(),
                                    );
                                }
                            },
                        }
                        BlueButton {
                            text: "Search".to_string(),
                            onclick: move |_| {
                                async move {
                                    let search_address = address_search_signal.read().to_string();
                                    if search_address.is_empty() {
                                        address_error_signal.set("Address cannot be empty!".to_string());
                                        error_signal.set("-".to_string());
                                        return;
                                    }
                                    console::log_1(&format!("Searching for address: {}", search_address).into());
                                    let result = get_address(&search_address).await;
                                    if result.is_err() {
                                        console::error_1(
                                            &format!(
                                                "Error getting coordinates: {}",
                                                result.err().expect("Expected error"),
                                            )
                                                .into(),
                                        );
                                        address_error_signal.set("No address found!".to_string());
                                        error_signal.set("-".to_string());
                                    } else {
                                        let address = result.expect("Expected coordinates");
                                        address_signal_addr
                                            .set(
                                                format!(
                                                    "{} {}, {}",
                                                    address.address.road.unwrap_or("-".to_string()),
                                                    address.address.house_number.unwrap_or("-".to_string()),
                                                    address.address.postcode.unwrap_or("-".to_string()),
                                                ),
                                            );
                                        address_signal_lat.set(address.lat.to_string());
                                        address_signal_lon.set(address.lon.to_string());
                                        address_error_signal.set("".to_string());
                                        is_no_error(
                                            team_name_error_signal,
                                            contact_email_error_signal,
                                            members_error_signal,
                                            address_error_signal,
                                            error_signal.clone(),
                                        );
                                    }
                                }
                            },
                        }
                        // Show Found Address
                        p { class: "mt-2 text-sm text-gray-700",
                            "Found Address: "
                            em { "{address_signal_addr}" }
                            em {
                                InputError { error_signal: address_error_signal.clone() }
                            }
                        }
                    }
                } else {
                    // Enter Coordinates
                    div { id: "coordinates",

                        label { class: "block font-semibold text-gray-700 mb-1", "Latitude" }

                        Input {
                            place_holer: Some("e.g. 50.1127197".to_string()),
                            value: address_signal_lat,
                            oninput: move |e: Event<FormData>| {
                                let lat = e.value();
                                match lat.parse::<f64>() {
                                    Ok(_) => {
                                        address_signal_lat.set(lat);
                                        address_error_signal.set("".to_string());
                                        is_no_error(
                                            team_name_error_signal,
                                            contact_email_error_signal,
                                            members_error_signal,
                                            address_error_signal,
                                            error_signal.clone(),
                                        );
                                    }
                                    Err(_) => {
                                        address_signal_lat.set(lat);
                                        address_error_signal.set("Invalid latitude!".to_string());
                                        error_signal.set("-".to_string());
                                    }
                                }
                            },
                        }


                        label { class: "block font-semibold text-gray-700 mb-1", "Longitude" }
                        Input {
                            place_holer: Some("e.g. 8.682092".to_string()),
                            value: address_signal_lon,
                            oninput: move |e: Event<FormData>| {
                                let lon = e.value();
                                match lon.parse::<f64>() {
                                    Ok(_) => {
                                        address_signal_lon.set(lon);
                                        address_error_signal.set("".to_string());
                                        is_no_error(
                                            team_name_error_signal,
                                            contact_email_error_signal,
                                            members_error_signal,
                                            address_error_signal,
                                            error_signal.clone(),
                                        );
                                    }
                                    Err(_) => {
                                        address_signal_lon.set(lon);
                                        address_error_signal.set("Invalid longitude!".to_string());
                                        error_signal.set("-".to_string());
                                    }
                                }
                            },
                        }

                        label { class: "block font-semibold text-gray-700 mb-1", "Address" }
                        Input {
                            place_holer: Some("e.g. Main Street 1, 12345 City".to_string()),
                            value: address_signal_addr,
                            oninput: move |e: Event<FormData>| {
                                let addr = e.value();
                                address_signal_addr.set(addr.clone());
                                if addr.is_empty() {
                                    address_error_signal.set("Address cannot be empty!".to_string());
                                    error_signal.set("-".to_string());
                                } else {
                                    address_error_signal.set("".to_string());
                                    is_no_error(
                                        team_name_error_signal,
                                        contact_email_error_signal,
                                        members_error_signal,
                                        address_error_signal,
                                        error_signal.clone(),
                                    );
                                }
                            },
                        }
                        InputError { error_signal: address_error_signal.clone() }
                    }
                }
            }
        }
    }
}

#[component]
fn TeamNotes(project_id: Uuid, props: TeamCardProps) -> Element {
    let mut create_note_headline_signal = use_signal(|| "".to_string());
    let mut create_note_headline_error_signal = use_signal(|| "".to_string());

    let mut create_note_description_signal = use_signal(|| "".to_string());
    let mut create_note_description_error_signal = use_signal(|| "".to_string());

    let mut create_note_error_signal = use_signal(|| "".to_string());

    let mut creating_error_signal = use_signal(|| "".to_string());

    let mut sorted_notes = props.notes.clone();
    sorted_notes.sort_by(|a, b| b.created.cmp(&a.created));

    let mut sorted_notes_signal = use_signal(|| sorted_notes);

    rsx! {
        div { class: "flex flex-col md:flex-row",
            // Left side: Team details
            div { class: "flex-1 pr-4 border-r border-gray-300", // Team name

                //Note
                label { class: "block font-semibold text-gray-700 mb-1", "Create Note" }
                Input {
                    place_holer: Some("e.g. Participation fee".to_string()),
                    value: create_note_headline_signal.clone(),
                    error_signal: create_note_headline_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let headline = e.value();
                        create_note_headline_signal.set(headline.clone());
                        if headline.is_empty() {
                            create_note_headline_error_signal
                                .set("Headline cannot be empty!".to_string());
                        } else if create_note_description_error_signal.read().is_empty() {
                            create_note_headline_error_signal.set("".to_string());
                            create_note_error_signal.set("".to_string());
                        } else {
                            create_note_headline_error_signal.set("".to_string());
                        }
                    },
                }
                InputError { error_signal: create_note_headline_error_signal.clone() }

                InputMultirow {
                    place_holer: Some("e.g. Participation fee is partially paid!".to_string()),
                    value: create_note_description_signal.clone(),
                    error_signal: create_note_description_error_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let description = e.value();
                        create_note_description_signal.set(description.clone());
                        if description.is_empty() {
                            create_note_description_error_signal
                                .set("Description cannot be empty!".to_string());
                            create_note_error_signal.set("-".to_string());
                        } else if create_note_headline_error_signal.read().is_empty() {
                            create_note_description_error_signal.set("".to_string());
                            create_note_error_signal.set("".to_string());
                        } else {
                            create_note_description_error_signal.set("".to_string());
                        }
                    },
                }

                InputError { error_signal: create_note_description_error_signal.clone() }

                GreenButton {
                    text: "Post Note".to_string(),
                    error_signal: create_note_error_signal,
                    onclick: move |_| {
                        if create_note_headline_signal.read().is_empty()
                            && create_note_description_signal.read().is_empty()
                        {
                            create_note_headline_error_signal
                                .set("Headline cannot be empty!".to_string());
                            create_note_description_error_signal
                                .set("Description cannot be empty!".to_string());
                            create_note_error_signal.set("-".to_string());
                            return;
                        } else if create_note_headline_signal.read().is_empty() {
                            create_note_headline_error_signal
                                .set("Headline cannot be empty!".to_string());
                            create_note_error_signal.set("-".to_string());
                            return;
                        } else if create_note_description_signal.read().is_empty() {
                            create_note_description_error_signal
                                .set("Description cannot be empty!".to_string());
                            create_note_error_signal.set("-".to_string());
                            return;
                        }
                        let result = add_team_note(
                            project_id,
                            props.id,
                            create_note_headline_signal.read().trim().to_string(),
                            create_note_description_signal.read().trim().to_string(),
                        );
                        if result.is_err() {
                            console::error_1(
                                &format!(
                                    "Error creating note: {}",
                                    result.err().expect("Expected error"),
                                )
                                    .into(),
                            );
                            creating_error_signal.set("Error creating note!".to_string());
                        } else {
                            sorted_notes_signal
                                .set({
                                    let mut note_list = vec![
                                        NoteProps {
                                            id: Uuid::new_v4(),
                                            headline: create_note_headline_signal
                                                .read()
                                                .trim()
                                                .to_string(),
                                            description: create_note_description_signal
                                                .read()
                                                .trim()
                                                .to_string(),
                                            created: chrono::Utc::now()
                                                .with_timezone(&chrono::Local)
                                                .format("%Y-%m-%d %H:%M")
                                                .to_string(),
                                        },
                                    ];
                                    note_list.extend(sorted_notes_signal.read().clone());
                                    note_list
                                });
                            create_note_headline_signal.set("".to_string());
                            create_note_description_signal.set("".to_string());
                            create_note_headline_error_signal.set("".to_string());
                            create_note_description_error_signal.set("".to_string());
                            create_note_error_signal.set("".to_string());
                            creating_error_signal.set("".to_string());
                        }
                    },
                }
                InputError { error_signal: creating_error_signal.clone() }
            }

            // Right side: Note block
            div { class: "flex-1 pl-4",
                label { class: "block font-semibold text-gray-700 mb-2", "Notes" }

                div { class: "space-y-2 overflow-y-auto max-h-96",
                    // Iterate over notes

                    for note in sorted_notes_signal.iter() {
                        div { class: "bg-white p-3 rounded-md shadow-sm",
                            div { class: "flex justify-between items-center",
                                h3 { class: "text-sm font-semibold text-gray-800",
                                    "{note.headline}"
                                }
                                p { class: "text-xs text-gray-500", "{note.created}" }
                            }
                            // Description below
                            p { class: "text-xs text-gray-600 mt-1", "{note.description}" }
                        }
                    }
                }
            

            }
        }
    }
}

fn check_all(
    team_name: &String,
    contact_email: &String,
    members: &String,
    address: &String,
    lat: &String,
    lon: &String,
    team_name_error_signal: Signal<String>,
    contact_email_error_signal: Signal<String>,
    members_error_signal: Signal<String>,
    address_error_signal: Signal<String>,
    error_signal: Signal<String>,
) -> bool {
    let is_team_name_valid = check_team_name(team_name, team_name_error_signal, error_signal);
    let is_contact_email_valid =
        check_contact_email(contact_email, contact_email_error_signal, error_signal);
    let is_members_valid = check_members(members, members_error_signal, error_signal);
    let is_address_valid = check_address(address, lat, lon, address_error_signal, error_signal);

    is_team_name_valid && is_contact_email_valid && is_members_valid && is_address_valid
}

fn check_team_name(
    team_name: &String,
    mut team_name_error_signal: Signal<String>,
    mut error_signal: Signal<String>,
) -> bool {
    if team_name.is_empty() {
        team_name_error_signal.set("Team name cannot be empty!".to_string());
        error_signal.set("-".to_string());
        return false;
    }
    team_name_error_signal.set("".to_string());
    true
}

fn check_contact_email(
    contact_email: &String,
    mut contact_email_error_signal: Signal<String>,
    mut error_signal: Signal<String>,
) -> bool {
    if contact_email.is_empty() {
        contact_email_error_signal.set("Contact E-Mail cannot be empty!".to_string());
        error_signal.set("-".to_string());
        return false;
    } else if !contact_email.contains('@') || !contact_email.contains('.') {
        contact_email_error_signal.set("Please enter a valid email address!".to_string());
        error_signal.set("-".to_string());
        return false;
    }
    contact_email_error_signal.set("".to_string());
    true
}

fn check_members(
    members: &String,
    mut members_error_signal: Signal<String>,
    mut error_signal: Signal<String>,
) -> bool {
    if members.is_empty() {
        members_error_signal.set("Number of Members cannot be empty!".to_string());
        error_signal.set("-".to_string());
        return false;
    } else if members.parse::<u32>().is_err() {
        members_error_signal.set("Please enter a valid number!".to_string());
        error_signal.set("-".to_string());
        return false;
    } else if members.parse::<u32>().unwrap() == 0 {
        members_error_signal.set("Number of Members must be greater than 0!".to_string());
        error_signal.set("-".to_string());
        return false;
    }
    members_error_signal.set("".to_string());
    true
}

fn check_address(
    address: &String,
    lat: &String,
    lon: &String,
    mut address_error_signal: Signal<String>,
    mut error_signal: Signal<String>,
) -> bool {
    if address.is_empty() {
        address_error_signal.set("Address cannot be empty!".to_string());
        error_signal.set("-".to_string());
        return false;
    }
    if lat.is_empty() || lon.is_empty() {
        address_error_signal.set("Latitude and Longitude cannot be empty!".to_string());
        error_signal.set("-".to_string());
        return false;
    }
    if lat.parse::<f64>().is_err() || lon.parse::<f64>().is_err() {
        address_error_signal.set("Invalid Latitude or Longitude!".to_string());
        error_signal.set("-".to_string());
        return false;
    }
    address_error_signal.set("".to_string());
    true
}

fn is_no_error(
    team_name_error_signal: Signal<String>,
    contact_email_error_signal: Signal<String>,
    members_error_signal: Signal<String>,
    address_error_signal: Signal<String>,
    mut error_signal: Signal<String>,
) {
    if team_name_error_signal.read().is_empty()
        && contact_email_error_signal.read().is_empty()
        && members_error_signal.read().is_empty()
        && address_error_signal.read().is_empty()
    {
        error_signal.set("".to_string());
    } else {
        error_signal.set("Please fix the errors before submitting.".to_string());
    }
}
