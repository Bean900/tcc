use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::address_connector::get_address;
use crate::storage::{ContactData, LocalStorage, NoteData};

use crate::{
    side::{BlueButton, CloseButton, GreenButton, Input, InputError, InputNumber},
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

            GreenButton { text: "Import team".to_string(), onclick: move |_| {} }
            BlueButton { text: "Share team link".to_string(), onclick: move |_| {} }


            // Scrollable grid
            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 p-6 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",


                {
                    props
                        .team_list
                        .iter()
                        .map(|team| {
                            let project_id = props.project_id;
                            let team_card_props = team.clone();
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
                                    class: "relative bg-white shadow-md rounded-xl p-6  hover:shadow-lg transition-all cursor-pointer hover:scale-105",
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
    pub created: DateTime<Utc>,
}

#[component]
fn TeamCard(props: &TeamCardProps) -> Element {
    rsx! {

        div {

            // Name
            h2 { class: "text-2xl font-semibold text-gray-800 mb-2", "{props.name}" }
            // Address
            p { class: "text-sm text-gray-600 mb-1", "📍 {props.address}" }
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
                h2 { class: "text-2xl font-semibold text-black-600 mb-4", "Add team" }
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
                                        "Error deleting project: {}",
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

    let allergies_signal = use_signal(|| "".to_string());

    let needs_check_signal = use_signal(|| team_card_props.needs_check);

    let notes_signal = use_signal(|| team_card_props.notes.clone());

    let error_signal = use_signal(|| "".to_string());
    check_all(
        &team_card_props.name,
        &team_card_props.contact_email,
        &team_card_props.members.to_string(),
        &team_card_props.address,
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
    rsx! {

        div { class: "backdrop-blur fixed inset-0 flex h-screen w-screen justify-center items-center",
            div { class: "relative bg-white shadow-md rounded-xl p-6 hover:shadow-lg transition-all cursor-pointer w-224",
                // Title
                h2 { class: "text-2xl font-semibold text-black-600 mb-4", "Add team" }
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
                                    notes: notes_signal
                                        .read()
                                        .iter()
                                        .map(|note| NoteData {
                                            id: note.id,
                                            headline: note.headline.clone(),
                                            description: note.description.clone(),
                                            created: note.created,
                                        })
                                        .collect(),
                                },
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
        // Address
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
                                spawn(async move {
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
                                });
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
