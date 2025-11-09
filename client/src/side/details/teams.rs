use dioxus::prelude::*;
use uuid::Uuid;
use web_sys::console;

use crate::side::details::address::{Address, AddressParam};
use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::{AddressSVG, Headline1, Headline2, InputPhoneNumber};
use crate::storage::{AddressData, NoteData, StorageManager, TeamCreate, TeamData};

use crate::side::{
    CloseButton, ConfirmButton, DeleteButton, Input, InputError, InputMultirow, InputNumber,
};

async fn add_team(
    id: Uuid,
    name: String,
    diets: Option<String>,
    mail: Option<String>,
    phone: Option<String>,
    members: Option<u32>,
    address: AddressData,
) -> Result<(), String> {
    let storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.read().clone();

    let team = TeamCreate {
        name,
        address,
        mail,
        diets,
        phone,
        members,
        needs_check: false,
    };
    storage
        .create_team_of_cook_and_run(id, Uuid::new_v4(), &team)
        .await
}

async fn update_team(id: Uuid, team: &TeamData) -> Result<(), String> {
    let storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.read().clone();
    storage.update_team_of_cook_and_run(id, team).await
}

async fn add_team_note(
    id: Uuid,
    team_id: Uuid,
    headline: String,
    description: String,
) -> Result<(), String> {
    let note = NoteData::new(headline, description);
    let storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.read().clone();
    storage
        .create_team_note_of_cook_and_run(id, team_id, &note)
        .await
}

async fn delete_team(id: Uuid, team_id: Uuid) -> Result<(), String> {
    let storage = use_context::<Signal<StorageManager>>();
    let mut storage = storage.read().clone();
    storage.delete_team_of_cook_and_run(id, team_id).await
}

#[component]
pub fn Teams(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let team_list: Resource<Result<Vec<TeamData>, String>> = use_resource(move || {
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let team_list = storage
                .select_cook_and_run_team_list(cook_and_run_id)
                .await?;
            Ok(team_list)
        }
    });

    match &*team_list.read_unchecked() {
        None => rsx!(
            LoadingPage {}
        ),
        Some(Err(e)) => rsx!(
            ErrorPage { error_text: e }
        ),
        Some(Ok(team_list)) => rsx!(
            TeamsContent { cook_and_run_id, team_list: team_list.clone() }
        ),
    }
}

enum PopUpWindow {
    None,
    AddTeam,
    EditTeam(TeamData),
}

#[component]
pub(crate) fn TeamsContent(cook_and_run_id: Uuid, team_list: Vec<TeamData>) -> Element {
    let mut team_dialog_signal: Signal<PopUpWindow> = use_signal(|| PopUpWindow::None);

    rsx! {
        section {
            Headline1 { headline: "Teams" }

            // Scrollable grid
            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 p-6 max-h-[calc(100vh-16rem)] overflow-y-auto pr-2",


                {
                    team_list
                        .iter()
                        .map(|team| {
                            let team_data = team.clone();
                            let background = if team.needs_check {
                                "bg-orange-100"
                            } else {
                                "bg-[#fdfaf6]"
                            };
                            rsx! {
                                a {
                                    key: {team.id},
                                    onclick: move |_| {
                                        team_dialog_signal.set(PopUpWindow::EditTeam(team_data.clone()));
                                    },
                                    class: "{background} relative  shadow-md rounded-xl p-6  hover:shadow-lg transition-all cursor-pointer hover:scale-105",
                                    {TeamCard(team.clone())}
                                }
                            }
                        })
                }

                a {
                    class: "border-4 border-dashed border-gray-300 rounded-xl p-6 flex items-center justify-center text-gray-400 hover:bg-[#fdfaf6] hover:text-[#C66741] hover:scale-105 transition-all duration-200 cursor-pointer",
                    onclick: move |_| {
                        team_dialog_signal.set(PopUpWindow::AddTeam);
                    },
                    div {

                        div { class: "text-5xl font-bold", "+" }
                    }
                }
            }
        }
        match &*team_dialog_signal.read() {
            PopUpWindow::None => rsx! {},
            PopUpWindow::AddTeam => rsx! {
                AddTeamDialog {
                    project_id: cook_and_run_id,
                    team_dialog_signal: team_dialog_signal.clone(),
                }
            },
            PopUpWindow::EditTeam(team_data) => {
                rsx! {
                    EditTeamDialog {
                        team_dialog_signal: team_dialog_signal.clone(),
                        project_id: cook_and_run_id,
                        team_data: team_data.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn TeamCard(props: TeamData) -> Element {
    rsx! {
        div {
            // Name
            Headline2 { headline: props.team_name.clone() }
            // Address
            div { class: "flex items-center space-x-2 mb-1",
                AddressSVG {}
                p { class: "text-sm text-gray-600 inline-flex items-center",
                    "{props.address.address}"
                }
            }
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
fn AddTeamDialog(team_dialog_signal: Signal<PopUpWindow>, project_id: Uuid) -> Element {
    let team_name_signal = use_signal(|| "".to_string());
    let team_name_error_signal = use_signal(|| "".to_string());

    let team_email_signal = use_signal(|| "".to_string());
    let team_email_error_signal = use_signal(|| "".to_string());

    let team_tel_signal = use_signal(|| "".to_string());
    let team_tel_error_signal = use_signal(|| "".to_string());

    let members_signal = use_signal(|| "".to_string());
    let members_error_signal = use_signal(|| "".to_string());

    let diets_signal = use_signal(|| "".to_string());

    let address_param = AddressParam::default();

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
                    team_email_signal,
                    team_email_error_signal,
                    team_tel_signal,
                    team_tel_error_signal,
                    members_signal,
                    members_error_signal,
                    diets_signal,
                    address_param: address_param.clone(),
                }
                // Close button
                CloseButton {
                    onclick: move |_| {
                        team_dialog_signal.set(PopUpWindow::None);
                    },
                }

                // Create team button
                div { class: "flex justify-center mt-4",
                    ConfirmButton {
                        text: "Create Team".to_string(),
                        onclick: move |_| {
                            async move {
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
                                let diets = (!diets_signal.read().trim().is_empty())
                                    .then_some(diets_signal.read().trim().to_string());
                                let tel = (!team_tel_signal.read().trim().is_empty())
                                    .then_some(team_tel_signal.read().trim().to_string());
                                let mail = (!team_email_signal.read().trim().is_empty())
                                    .then_some(team_email_signal.read().trim().to_string());
                                let members = members_signal
                                    .read()
                                    .parse::<u32>()
                                    .map_or_else(|_| None, |v| Some(v));
                                let result = add_team(
                                        project_id,
                                        team_name_signal.read().trim().to_string(),
                                        diets,
                                        mail,
                                        tel,
                                        members,
                                        address_param
                                            .get_address_data()
                                            .expect("Expext no errors when getting address_data!"),
                                    )
                                    .await;
                                if let Err(e) = result {
                                    console::error_1(&format!("Error creating team: {}", e).into());
                                } else {
                                    team_dialog_signal.set(PopUpWindow::None);
                                }
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
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    team_data: TeamData,
) -> Element {
    let team_name_signal = use_signal(|| team_data.team_name.clone());
    let team_name_error_signal = use_signal(|| "".to_string());

    let team_email_signal = use_signal(|| team_data.mail.clone());
    let team_email_error_signal = use_signal(|| "".to_string());

    let team_tel_signal = use_signal(|| team_data.phone_number.clone());
    let team_tel_error_signal = use_signal(|| "".to_string());

    let members_signal = use_signal(|| team_data.members.to_string());
    let members_error_signal = use_signal(|| "".to_string());

    let diets_signal = use_signal(|| team_data.diets.join(", "));

    let mut needs_check_signal = use_signal(|| team_data.needs_check);

    let error_signal = use_signal(|| "".to_string());

    let mut is_edit_team_signal = use_signal(|| true);

    let address_param = AddressParam::new(&team_data.address);

    use_effect(move || {
        check_all(
            team_name_signal,
            team_name_error_signal,
            team_email_signal,
            team_email_error_signal,
            team_tel_signal,
            team_tel_error_signal,
            members_error_signal,
            members_signal,
            address_param,
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
                            class: if *is_edit_team_signal.read() { "px-4 py-2 font-semibold text-sm text-[#C66741] border-b-2 border-[#C66741]" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-[#C66741]" },
                            "Team Data"
                        }
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                is_edit_team_signal.set(false);
                            },
                            class: if !*is_edit_team_signal.read() { "px-4 py-2 font-semibold text-sm text-[#C66741] border-b-2 border-[#C66741]" } else { "px-4 py-2 font-semibold text-sm text-gray-600 hover:text-[#C66741]" },
                            "Notes"
                        }
                    }
                    div {
                        div { class: "flex items-center space-x-2",
                            label { class: "text-sm text-gray-600", "Team needs check:" }
                            input {
                                r#type: "checkbox",
                                checked: needs_check_signal,
                                class: "text-[#C66741] rounded",
                                onclick: move |_| {
                                    let new_value = !*needs_check_signal.read();
                                    team_data.needs_check = new_value;
                                    todo!();
                                },
                            }
                        }
                    }
                    DeleteButton {
                        onclick: move |_| {
                            async move {
                                let result = delete_team(project_id, team_data.id).await;
                                if let Err(e) = result {
                                    console::error_1(&format!("Error deleting team: {}", e).into());
                                } else {
                                    team_dialog_signal.set(PopUpWindow::None);
                                }
                            }
                        },
                    }
                }

                // Close button
                CloseButton {
                    onclick: move |_| {
                        team_dialog_signal.set(PopUpWindow::None);
                    },
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
                        address_param,
                    }


                    // Create team button
                    div { class: "flex justify-center mt-4",
                        ConfirmButton {
                            text: "Update Team".to_string(),
                            error_signal: error_signal.clone(),
                            onclick: move |_| {
                                async move {
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
                                    let result = update_team(
                                            project_id,
                                            &TeamData {
                                                id: team_data.id,
                                                team_name: team_name_signal.read().trim().to_string(),
                                                address: address_param
                                                    .get_address_data()
                                                    .expect("Expect address data!"),
                                                mail: team_email_signal.read().trim().to_string(),
                                                phone_number: team_tel_signal.read().trim().to_string(),
                                                members: members_signal.read().parse::<u32>().unwrap_or(0),
                                                diets: diets_signal
                                                    .read()
                                                    .split(',')
                                                    .map(|s| s.trim().to_string())
                                                    .collect(),
                                                needs_check: *needs_check_signal.read(),
                                                note_list: vec![],
                                            },
                                        )
                                        .await;
                                    if result.is_err() {
                                        console::error_1(
                                            &format!(
                                                "Error updating team: {}",
                                                result.err().expect("Expected error"),
                                            )
                                                .into(),
                                        );
                                    } else {
                                        team_dialog_signal.set(PopUpWindow::None);
                                    }
                                }
                            },
                        }
                    }
                } else {
                    TeamNotes {
                        project_id,
                        team_id: team_data.id,
                        note_data_list: team_data.note_list,
                    }
                }
            }
        }
    }
}

#[component]
fn TeamDialog(
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    team_name_signal: Signal<String>,
    team_name_error_signal: Signal<String>,
    team_email_signal: Signal<String>,
    team_email_error_signal: Signal<String>,
    team_tel_signal: Signal<String>,
    team_tel_error_signal: Signal<String>,
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

                // Team E-Mail
                label { class: "block font-semibold text-gray-700 mb-1", "Team E-Mail" }
                Input {
                    place_holer: Some("e.g. chili@chasers.de".to_string()),
                    is_error: !team_email_error_signal.read().is_empty(),
                    value: team_email_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let team_email = e.value();
                        team_email_signal.set(team_email.clone());
                        check_team_email(team_email_signal, team_email_error_signal);
                    },
                }
                InputError { error: team_email_error_signal.read() }

                // Team Phone Number
                label { class: "block font-semibold text-gray-700 mb-1", "Team Phone Number" }
                InputPhoneNumber {
                    place_holer: Some("e.g. +49 1234 56789".to_string()),
                    is_error: !team_tel_error_signal.read().is_empty(),
                    value: team_tel_signal.clone(),
                    oninput: move |e: Event<FormData>| {
                        let team_tel = e.value();
                        team_tel_signal.set(team_tel.clone());
                        check_team_tel(team_tel_signal, team_tel_error_signal);
                    },
                }
                InputError { error: team_tel_error_signal.read() }

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

#[component]
fn TeamNotes(project_id: Uuid, team_id: Uuid, note_data_list: Vec<NoteData>) -> Element {
    let mut create_note_headline_signal = use_signal(|| "".to_string());
    let mut create_note_headline_error_signal = use_signal(|| "".to_string());

    let mut create_note_description_signal = use_signal(|| "".to_string());
    let mut create_note_description_error_signal = use_signal(|| "".to_string());

    let mut create_note_error_signal = use_signal(|| "".to_string());

    let mut creating_error_signal = use_signal(|| "".to_string());

    let mut sorted_note_list = note_data_list.clone();
    sorted_note_list.sort_by(|a, b| b.created.cmp(&a.created));

    let mut sorted_note_list_signal = use_signal(|| sorted_note_list);

    rsx! {
        div { class: "flex flex-col md:flex-row",
            // Left side: Team details
            div { class: "flex-1 pr-4 border-r border-gray-300", // Team name

                //Note
                label { class: "block font-semibold text-gray-700 mb-1", "Create Note" }
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
                        } else if create_note_description_error_signal.read().is_empty() {
                            create_note_headline_error_signal.set("".to_string());
                            create_note_error_signal.set("".to_string());
                        } else {
                            create_note_headline_error_signal.set("".to_string());
                        }
                    },
                }
                InputError { error: create_note_headline_error_signal.read() }

                InputMultirow {
                    place_holer: Some("e.g. Participation fee is partially paid!".to_string()),
                    value: create_note_description_signal.clone(),
                    is_error: !create_note_description_error_signal.read().is_empty(),
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

                InputError { error: create_note_description_error_signal.read() }

                ConfirmButton {
                    text: "Post Note".to_string(),
                    error_signal: create_note_error_signal,
                    onclick: move |_| {
                        async move {
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
                                    team_id,
                                    create_note_headline_signal.read().trim().to_string(),
                                    create_note_description_signal.read().trim().to_string(),
                                )
                                .await;
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
                                sorted_note_list_signal
                                    .set({
                                        let mut note_list = vec![
                                            NoteData {
                                                id: Uuid::new_v4(),
                                                headline: create_note_headline_signal
                                                    .read()
                                                    .trim()
                                                    .to_string(),
                                                description: create_note_description_signal
                                                    .read()
                                                    .trim()
                                                    .to_string(),
                                                created: chrono::Utc::now(),
                                            },
                                        ];
                                        note_list.extend(sorted_note_list_signal.read().clone());
                                        note_list
                                    });
                                create_note_headline_signal.set("".to_string());
                                create_note_description_signal.set("".to_string());
                                create_note_headline_error_signal.set("".to_string());
                                create_note_description_error_signal.set("".to_string());
                                create_note_error_signal.set("".to_string());
                                creating_error_signal.set("".to_string());
                            }
                        }
                    },
                }
                InputError { error: creating_error_signal.read() }
            }

            // Right side: Note block
            div { class: "flex-1 pl-4",
                label { class: "block font-semibold text-gray-700 mb-2", "Notes" }

                div { class: "space-y-2 overflow-y-auto max-h-96",
                    // Iterate over note_list
                    for note_data in sorted_note_list_signal.iter() {
                        Note { note_data: note_data.clone() }
                    }
                }
            
            }
        }
    }
}

#[component]
fn Note(note_data: NoteData) -> Element {
    let created = note_data
        .created
        .with_timezone(&chrono::Local)
        .format("%Y-%m-%d %H:%M")
        .to_string();
    rsx!(
        div { class: "bg-white p-3 rounded-md shadow-sm",
            div { class: "flex justify-between items-center",
                h3 { class: "text-sm font-semibold text-gray-800", "{note_data.headline}" }
                p { class: "text-xs text-gray-500", "{created}" }
            }
            // Description below
            p { class: "text-xs text-gray-600 mt-1", "{note_data.description}" }
        }
    )
}

fn check_all(
    team_name_signal: Signal<String>,
    team_name_error_signal: Signal<String>,
    team_email_signal: Signal<String>,
    team_email_error_signal: Signal<String>,
    team_tel_signal: Signal<String>,
    team_tel_error_signal: Signal<String>,
    members_error_signal: Signal<String>,
    members_signal: Signal<String>,
    address_param: AddressParam,
) -> bool {
    let team_name_check = check_team_name(team_name_signal, team_name_error_signal);
    let team_email_check = check_team_email(team_email_signal, team_email_error_signal);
    let team_tel_check = check_team_tel(team_tel_signal, team_tel_error_signal);
    let member_check = check_members(members_signal, members_error_signal);
    let address_check = address_param.check_address_data().is_ok();
    team_name_check && team_email_check && team_tel_check && member_check && address_check
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

fn check_team_email(
    team_email_signal: Signal<String>,
    mut team_email_error_signal: Signal<String>,
) -> bool {
    let team_email = team_email_signal.read();
    if team_email.is_empty() {
        team_email_error_signal.set("Team E-Mail cannot be empty!".to_string());
        false
    } else if !team_email.contains('@') || !team_email.contains('.') {
        team_email_error_signal.set("Please enter a valid email address!".to_string());
        false
    } else {
        team_email_error_signal.set("".to_string());
        true
    }
}

fn check_team_tel(
    team_tel_signal: Signal<String>,
    mut team_tel_error_signal: Signal<String>,
) -> bool {
    let team_tel = team_tel_signal.read();
    if team_tel.is_empty() {
        team_tel_error_signal.set("Team phone number cannot be empty!".to_string());
        false
    } else {
        team_tel_error_signal.set("".to_string());
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
