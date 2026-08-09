use async_std::task::sleep;
use std::time::Duration;

use crate::side::details::address::{Address, AddressParam};
use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::{AddressSVG};
use crate::storage::{
    AddressData, NoteCreate, NoteData, ShareTeamConfig, StorageManager, TeamCreate, TeamData,
    TeamUpdate,
};
use crate::ui::forms::InputPhoneNumber;
use crate::{async_action, storage};
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{DateTime, Local, Utc};
use dioxus::prelude::*;
use qrcode::render::svg;
use qrcode::QrCode;
use uuid::Uuid;
use web_sys::console;
use web_sys::wasm_bindgen::JsCast;

use crate::ui::{
    buttons::{AsyncAction, CloseButton, ConfirmButton, DeleteButtonProps, PrimaryButton},
    cards::SearchFilterCard,
    forms::{Input, InputError, InputNumber, TextArea},
    icons::PlusIcon,
    typography::Headline1,
};

// ─────────────────────────────────────────────
//  Shared design tokens & Sort Options
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

const NATIVE_INPUT: &str = "w-full px-3 py-2 rounded-xl border border-amber-200 bg-amber-50/40 \
     text-sm text-zinc-800 placeholder-zinc-400 \
     focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 \
     transition-colors duration-150";

const SAVE_GLOW_CSS: &str = r#"
@keyframes save-glow {
    0%   { border-color:#d1fae5; box-shadow:0 0 0 0px rgba(34,197,94,0),   0 1px 3px 0 rgba(0,0,0,0.06); }
    20%  { border-color:#22c55e; box-shadow:0 0 0 5px rgba(34,197,94,0.22),0 1px 3px 0 rgba(0,0,0,0.06); }
    55%  { border-color:#16a34a; box-shadow:0 0 0 5px rgba(34,197,94,0.10),0 1px 3px 0 rgba(0,0,0,0.06); }
    100% { border-color:#bbf7d0; box-shadow:0 0 0 0px rgba(34,197,94,0),   0 1px 3px 0 rgba(0,0,0,0.06); }
}
.save-glow-dialog { animation: save-glow 2s ease-in-out forwards; }
.save-glow-dialog .save-glow-header {
    background-color: rgba(240,253,244,0.70) !important;
    border-bottom-color: #bbf7d0 !important;
    transition: background-color 0.4s ease, border-color 0.4s ease;
}
.save-glow-dialog .save-glow-accent {
    background-color: rgba(34,197,94,0.75) !important;
    transition: background-color 0.4s ease;
}
.save-glow-dialog .save-glow-title {
    color: #166534 !important;
    transition: color 0.4s ease;
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamSortOption {
    NameAsc,
    NameDesc,
    MembersDesc,
}

fn tab_cls(active: bool) -> &'static str {
    if active {
        "px-4 py-2 text-sm font-semibold text-[#C66741] border-b-2 border-[#C66741] -mb-px"
    } else {
        "px-4 py-2 text-sm font-medium text-zinc-500 hover:text-[#C66741] transition-colors"
    }
}

// ─────────────────────────────────────────────
//  Helpers
// ─────────────────────────────────────────────

fn map_string(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn map_u8(value: String) -> Option<u8> {
    value.trim().parse::<u8>().ok()
}

async fn add_team(
    id: Uuid,
    name: String,
    diets: String,
    mail: String,
    phone: String,
    members: String,
    address: AddressData,
) -> Result<(), String> {
    let team = TeamCreate {
        name,
        address,
        mail: map_string(mail),
        diets: map_string(diets),
        phone: map_string(phone),
        members: map_u8(members),
        needs_check: false,
    };
    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let mut storage = storage_signal.write();
    storage
        .create_team_of_cook_and_run(id, Uuid::new_v4(), &team)
        .await
}

async fn update_team(
    id: Uuid,
    team_id: Uuid,
    name: String,
    diets: String,
    mail: String,
    phone: String,
    members: String,
    address: AddressData,
    needs_check: bool,
) -> Result<(), String> {
    let team = TeamUpdate {
        name,
        address,
        mail: map_string(mail),
        diets: map_string(diets),
        phone: map_string(phone),
        members: map_u8(members),
        needs_check,
    };
    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let mut storage = storage_signal.write();
    storage
        .update_team_of_cook_and_run(id, team_id, &team)
        .await
}

async fn add_team_note(
    id: Uuid,
    team_id: Uuid,
    headline: String,
    content: String,
) -> Result<(), String> {
    let note = NoteCreate { headline, content };
    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let mut storage = storage_signal.write();
    storage
        .create_team_note_of_cook_and_run(id, team_id, Uuid::new_v4(), &note)
        .await
}

async fn delete_team(id: Uuid, team_id: Uuid) -> Result<(), String> {
    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let mut storage = storage_signal.write();
    storage.delete_team_of_cook_and_run(id, team_id).await
}

// ─────────────────────────────────────────────
//  Root
// ─────────────────────────────────────────────

#[component]
pub fn Teams(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();
    let team_list: Resource<Result<(Vec<TeamData>, bool), String>> = use_resource(move || {
        let storage = storage;
        async move {
            let storage = storage.read().clone();
            let team_list = storage
                .select_cook_and_run_team_list(cook_and_run_id)
                .await?;
            let meta = storage.select_cook_and_run_meta(cook_and_run_id).await?;
            Ok((team_list, meta.is_in_cloud))
        }
    });

    let list_ref = team_list.read();
    match &*list_ref {
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
        Some(Ok((teams, is_cloud))) => rsx!(
            TeamsContent {
                cook_and_run_id,
                team_list: teams.clone(),
                is_online: *is_cloud,
            }
        ),
    }
}

// ─────────────────────────────────────────────
//  Popup state
// ─────────────────────────────────────────────

enum PopUpWindow {
    None,
    AddTeam,
    EditTeam(TeamData),
    Share(Option<ShareTeamConfig>),
}

#[derive(Debug, Clone, PartialEq)]
enum ShareConfigState {
    Offline,
    Loaded(ShareTeamConfig),
    None,
}

// ─────────────────────────────────────────────
//  Teams content
// ─────────────────────────────────────────────

#[component]
pub(crate) fn TeamsContent(
    cook_and_run_id: Uuid,
    team_list: Vec<TeamData>,
    is_online: bool,
) -> Element {
    let number_of_teams = team_list.len();
    let mut team_dialog_signal: Signal<PopUpWindow> = use_signal(|| PopUpWindow::None);

    let mut search_signal = use_signal(String::new);
    let mut sort_signal = use_signal(|| TeamSortOption::NameAsc);

    let storage = use_context::<Signal<StorageManager>>();
    let share_config: Resource<Result<ShareConfigState, String>> =
        use_resource(move || async move {
            if !is_online {
                return Ok(ShareConfigState::Offline);
            }
            let storage = storage.read().clone();
            let share_config = storage
                .select_cook_and_run_share_config(cook_and_run_id)
                .await?;
            match share_config {
                Some(config) => Ok(ShareConfigState::Loaded(config)),
                None => Ok(ShareConfigState::None),
            }
        });

    let share_icon_svg = rsx! {
        svg {
            class: "w-5 h-5",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            circle { cx: "18", cy: "5", r: "3" }
            circle { cx: "6", cy: "12", r: "3" }
            circle { cx: "18", cy: "19", r: "3" }
            line {
                x1: "8.59",
                y1: "13.51",
                x2: "15.42",
                y2: "17.49",
            }
            line {
                x1: "15.41",
                y1: "6.51",
                x2: "8.59",
                y2: "10.49",
            }
        }
    };

    let share_btn_class = "flex items-center justify-center w-9 h-9 rounded-[10px] border-none cursor-pointer bg-[#D67229] hover:bg-[#C66741] text-white shadow-[0_2px_8px_rgba(214,114,41,0.3)] hover:shadow-[0_4px_14px_rgba(198,103,65,0.45)] transition-all duration-150";
    let add_team_class = "border-2 border-dashed border-[#D67229]/35 hover:border-[#D67229] rounded-2xl p-5 flex flex-col items-center justify-center gap-2.5 cursor-pointer text-[#D67229]/50 hover:text-[#C66741] bg-[#FDF6EC]/30 hover:bg-[#FDF6EC]/80 transition-all duration-150 min-h-[120px]";

    let sort_options = vec![
        (TeamSortOption::NameAsc, "Name (A–Z)".to_string()),
        (TeamSortOption::NameDesc, "Name (Z–A)".to_string()),
        (TeamSortOption::MembersDesc, "Mitglieder".to_string()),
    ];

    let query = search_signal.read().trim().to_lowercase();
    let mut filtered_teams: Vec<_> = team_list
        .iter()
        .filter(|t| {
            query.is_empty()
                || t.name.to_lowercase().contains(&query)
                || t.address.address.to_lowercase().contains(&query)
                || t.mail.as_deref().unwrap_or("").to_lowercase().contains(&query)
        })
        .cloned()
        .collect();

    match *sort_signal.read() {
        TeamSortOption::NameAsc => {
            filtered_teams.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }
        TeamSortOption::NameDesc => {
            filtered_teams.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()))
        }
        TeamSortOption::MembersDesc => {
            filtered_teams.sort_by(|a, b| b.members.unwrap_or(0).cmp(&a.members.unwrap_or(0)))
        }
    }

    let config = match &*share_config.read() {
        None => rsx! {
            div { class: "w-9 h-9 rounded-xl bg-amber-100/60 animate-pulse" }
        },
        Some(Err(e)) => rsx! {
            div {
                class: "flex items-center justify-center w-9 h-9 rounded-xl bg-red-50 border border-red-200 text-red-400",
                title: "Error loading share config: {e}",
                svg {
                    class: "w-4 h-4",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    line {
                        x1: "18",
                        y1: "6",
                        x2: "6",
                        y2: "18",
                    }
                    line {
                        x1: "6",
                        y1: "6",
                        x2: "18",
                        y2: "18",
                    }
                }
            }
        },
        Some(Ok(ShareConfigState::Offline)) => rsx! {
            div {
                class: "flex items-center justify-center w-9 h-9 rounded-xl bg-zinc-100 border border-zinc-200 text-zinc-300 cursor-not-allowed",
                title: "Sharing is only available when the project is stored in the cloud.",
                button {
                    class: "flex items-center justify-center w-full h-full",
                    disabled: true,
                    {share_icon_svg}
                }
            }
        },
        Some(Ok(ShareConfigState::Loaded(config))) => {
            let config = config.clone();
            rsx! {
                button {
                    class: "{share_btn_class}",
                    onclick: move |_| {
                        team_dialog_signal.set(PopUpWindow::Share(Some(config.clone())));
                    },
                    {share_icon_svg}
                }
            }
        }
        Some(Ok(ShareConfigState::None)) => rsx! {
            button {
                class: "{share_btn_class}",
                onclick: move |_| {
                    team_dialog_signal.set(PopUpWindow::Share(None));
                },
                {share_icon_svg}
            }
        },
    };

    rsx! {
        section { class: "max-w-[1800px] mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8 space-y-6",
            // ── Header Bar (Mobil optimiert mit direktem Add Team Button) ──
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 border-b border-amber-100/80 pb-3",
                Headline1 {
                    headline: "Teams".to_string(),
                    subtitle: "Manage your teams and configuration, so that everyone can create their own teams."
                        .to_string(),
                }
                div { class: "flex items-center gap-3 shrink-0 self-start sm:self-auto",
                    PrimaryButton {
                        text: "New Team".to_string(),
                        icon: rsx! {
                            PlusIcon { class: "w-4 h-4 text-white".to_string() }
                        },
                        onclick: move |_| team_dialog_signal.set(PopUpWindow::AddTeam),
                    }
                    {config}
                }
            }

            if !team_list.is_empty() {
                SearchFilterCard {
                    search_value: search_signal.read().clone(),
                    search_placeholder: Some("Search teams…".to_string()),
                    on_search_change: move |val| search_signal.set(val),
                    sort_value: *sort_signal.read(),
                    sort_options,
                    on_sort_change: move |val| sort_signal.set(val),
                    use_card_wrapper: true,
                }
            }

            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 max-h-[calc(100vh-18rem)] overflow-y-auto pr-1",
                {
                    filtered_teams
                        .iter()
                        .map(|team| {
                            let team_data = team.clone();
                            let card_cls = if team.needs_check {
                                "relative bg-white rounded-2xl border border-amber-400 shadow-xs hover:shadow-md transition-all duration-150 cursor-pointer overflow-hidden"
                            } else {
                                "relative bg-white rounded-2xl border border-amber-100/80 shadow-xs hover:shadow-md transition-all duration-150 cursor-pointer overflow-hidden"
                            };
                            rsx! {
                                a {
                                    key: "{team.id}",
                                    onclick: move |_| {
                                        team_dialog_signal.set(PopUpWindow::EditTeam(team_data.clone()));
                                    },
                                    class: "{card_cls}",
                                    TeamCard {
                                        name: team.name.clone(),
                                        address: team.address.address.clone(),
                                        needs_check: team.needs_check,
                                    }
                                }
                            }
                        })
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
            PopUpWindow::EditTeam(team_data) => rsx! {
                EditTeamDialog {
                    team_dialog_signal: team_dialog_signal.clone(),
                    project_id: cook_and_run_id,
                    team_data: team_data.clone(),
                }
            },
            PopUpWindow::Share(share_config) => rsx! {
                ShareDialog {
                    team_dialog_signal: team_dialog_signal.clone(),
                    project_id: cook_and_run_id,
                    share_config_option: share_config.clone(),
                    number_of_teams,
                }
            },
        }
    }
}

// ─────────────────────────────────────────────
//  Team card (grid tile)
// ─────────────────────────────────────────────

#[derive(Props, Clone, PartialEq)]
struct TeamCardProps {
    name: String,
    address: String,
    needs_check: bool,
}

#[component]
fn TeamCard(props: TeamCardProps) -> Element {
    let needs_check = props.needs_check;
    let style_0 = if needs_check {
        "bg-amber-100/80 border-amber-300"
    } else {
        "bg-amber-50/70 border-amber-100"
    };
    let style_1 = if needs_check {
        "bg-amber-500"
    } else {
        "bg-amber-400/70"
    };

    rsx! {
        div {
            div { class: "px-4 py-2.5 border-b flex items-center gap-2 {style_0}",
                div { class: "w-1.5 h-4 rounded-full shrink-0 {style_1}" }
                span { class: "text-sm font-semibold text-zinc-800 truncate", "{props.name}" }
                if needs_check {
                    span { class: "ml-auto text-[10px] font-bold px-1.5 py-0.5 rounded-full bg-amber-500 text-white tracking-wide shrink-0",
                        "REVIEW"
                    }
                }
            }
            div { class: "px-4 py-3",
                div { class: "flex items-center gap-1.5",
                    AddressSVG {}
                    p { class: "text-xs text-zinc-500 truncate", "{props.address}" }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Add team dialog
// ─────────────────────────────────────────────

#[component]
fn AddTeamDialog(team_dialog_signal: Signal<PopUpWindow>, project_id: Uuid) -> Element {
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
        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen justify-center items-center bg-black/20 z-50 p-4",
            div { class: "relative bg-white rounded-2xl border border-amber-100 shadow-xl w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden",
                // Stabile Header-Leiste
                div { class: "px-6 py-4 bg-amber-50/70 border-b border-amber-100 flex items-center justify-between gap-2.5 shrink-0 relative",
                    div { class: "flex items-center gap-2.5",
                        div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                        span { class: "text-base font-semibold text-zinc-800", "Add Team" }
                    }
                    CloseButton {
                        onclick: move |_| {
                            team_dialog_signal.set(PopUpWindow::None);
                        },
                    }
                }

                // Scrollbarer Inhaltsbereich
                div { class: "px-6 py-5 overflow-y-auto flex-1",
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
                                { if ! check_all(team_name_signal, team_name_error_signal, team_email_signal,
                                team_email_error_signal, team_tel_signal, team_tel_error_signal,
                                members_error_signal, members_signal, address_param.clone(),) { return; } let
                                address_data = address_param.get_address_data()
                                .expect("Expect no errors when getting address_data!"); let result =
                                add_team(project_id, team_name_signal.read().clone(), diets_signal.read()
                                .clone(), team_email_signal.read().clone(), team_tel_signal.read().clone(),
                                members_signal.read().clone(), address_data,). await; if let Err(e) = result {
                                console::error_1(& format!("Error creating team: {e}") .into()); } else {
                                team_dialog_signal.set(PopUpWindow::None); } }
                            ),
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Edit team dialog
// ─────────────────────────────────────────────

#[component]
fn EditTeamDialog(
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    team_data: TeamData,
) -> Element {
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
            address_param.clone(),
        );
    });

    let delete_button = DeleteButtonProps::new(
        async_action!({
            if let Err(e) = delete_team(project_id, team_data.id).await {
                console::error_1(&format!("Error deleting team: {e}").into());
            } else {
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

    let dialog_class = if is_success {
        "relative bg-white rounded-2xl border shadow-xl w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden save-glow-dialog"
    } else {
        "relative bg-white rounded-2xl border border-amber-100 shadow-xl w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden"
    };
    let header_class = if is_success {
        "px-6 py-4 border-b flex items-center justify-between gap-2.5 shrink-0 relative save-glow-header"
    } else {
        "px-6 py-4 bg-amber-50/70 border-b border-amber-100 flex items-center justify-between gap-2.5 shrink-0 relative"
    };
    let accent_class = if is_success {
        "w-1.5 h-5 rounded-full save-glow-accent"
    } else {
        "w-1.5 h-5 rounded-full bg-amber-400/70"
    };
    let title_class = if is_success {
        "text-base font-semibold save-glow-title"
    } else {
        "text-base font-semibold text-zinc-800"
    };

    rsx! {
        style { dangerous_inner_html: SAVE_GLOW_CSS }

        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen justify-center items-center bg-black/20 z-50 p-4",
            div { class: "{dialog_class}",
                // Stabile Header-Leiste
                div { class: "{header_class}",
                    div { class: "flex items-center gap-2.5",
                        div { class: "{accent_class}" }
                        span { class: "{title_class}", "Edit Team" }
                    }
                    CloseButton {
                        onclick: move |_| {
                            team_dialog_signal.set(PopUpWindow::None);
                        },
                    }
                }

                // Scrollbarer Inhaltsbereich
                div { class: "px-6 py-5 overflow-y-auto flex-1",
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
                            label { class: "flex items-center gap-2 cursor-pointer group",
                                input {
                                    r#type: "checkbox",
                                    checked: needs_check_signal,
                                    class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                    onclick: move |_| {
                                        let new_value = !*needs_check_signal.read();
                                        needs_check_signal.set(new_value);
                                        has_unsaved_changes.set(true);
                                    },
                                }
                                span { class: "text-[13px] text-zinc-500 group-hover:text-zinc-700 transition-colors",
                                    "Needs review"
                                }
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
                                    error_signal: error_signal.clone(),
                                    action: async_action!(
                                        { if ! check_all(team_name_signal, team_name_error_signal, team_email_signal,
                                        team_email_error_signal, team_tel_signal, team_tel_error_signal,
                                        members_error_signal, members_signal, address_param.clone(),) { return; } let
                                        address_data = address_param.get_address_data()
                                        .expect("Expect no errors when getting address_data!"); let result =
                                        update_team(project_id, team_data.id, team_name_signal.read().clone(),
                                        diets_signal.read().clone(), team_email_signal.read().clone(), team_tel_signal
                                        .read().clone(), members_signal.read().clone(), address_data, *
                                        needs_check_signal.read(),). await; if let Err(e) = result { console::error_1(&
                                        format!("Error updating team: {e}") .into()); } else { save_success_signal
                                        .set(true); spawn(async move { sleep(Duration::from_millis(2000)). await;
                                        team_dialog_signal.set(PopUpWindow::None); }); } }
                                    ),
                                }
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
}

// ─────────────────────────────────────────────
//  Shared team form (Add + Edit)
// ─────────────────────────────────────────────

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
    on_change: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-col md:flex-row gap-6",
            div { class: "flex-1 space-y-3 md:border-r border-amber-100 md:pr-5",
                div {
                    label { class: "{LBL}", "Team Name" }
                    Input {
                        place_holer: Some("e.g. The Chili Chasers".to_string()),
                        is_error: !team_name_error_signal.read().is_empty(),
                        value: team_name_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let team_name = e.value();
                            team_name_signal.set(team_name);
                            check_team_name(team_name_signal, team_name_error_signal);
                            on_change.call(());
                        },
                    }
                    InputError { error: team_name_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Email" }
                    Input {
                        place_holer: Some("e.g. chili@chasers.de".to_string()),
                        is_error: !team_email_error_signal.read().is_empty(),
                        value: team_email_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let team_email = e.value();
                            team_email_signal.set(team_email);
                            check_team_email(team_email_signal, team_email_error_signal);
                            on_change.call(());
                        },
                    }
                    InputError { error: team_email_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Phone Number" }
                    InputPhoneNumber {
                        placeholder: Some("e.g. +49 1234 56789".to_string()),
                        is_error: !team_tel_error_signal.read().is_empty(),
                        value: team_tel_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let team_tel = e.value();
                            team_tel_signal.set(team_tel);
                            check_team_tel(team_tel_signal, team_tel_error_signal);
                            on_change.call(());
                        },
                    }
                    InputError { error: team_tel_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Number of Members" }
                    InputNumber {
                        placeholder: Some("e.g. 2".to_string()),
                        value: members_signal.clone(),
                        is_error: !members_error_signal.read().is_empty(),
                        oninput: move |e: Event<FormData>| {
                            let members = e.value();
                            members_signal.set(members);
                            check_members(members_signal, members_error_signal);
                            on_change.call(());
                        },
                    }
                    InputError { error: members_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Dietary Requirements" }
                    div { class: "w-full",
                        Input {
                            place_holer: Some("e.g. vegetarian, nut allergy, halal ...".to_string()),
                            is_error: false,
                            value: diets_signal.clone(),
                            oninput: move |e: Event<FormData>| {
                                diets_signal.set(e.value());
                                on_change.call(());
                            },
                        }
                    }
                }
            }

            div { class: "flex-1",
                label { class: "{LBL}", "Address" }
                Address { param: address_param }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Team notes tab
// ─────────────────────────────────────────────

#[component]
fn TeamNotes(project_id: Uuid, team_id: Uuid, note_data_list: Vec<NoteData>) -> Element {
    let mut create_note_headline_signal = use_signal(String::new);
    let mut create_note_headline_error_signal = use_signal(String::new);
    let mut create_note_content_signal = use_signal(String::new);
    let mut create_note_content_error_signal = use_signal(String::new);
    let mut create_note_error_signal = use_signal(String::new);
    let mut creating_error_signal = use_signal(String::new);

    let mut sorted_note_list = note_data_list;
    sorted_note_list.sort_by(|a, b| b.created.cmp(&a.created));
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
                        { let headline = create_note_headline_signal.read().trim().to_string(); let
                        content = create_note_content_signal.read().trim().to_string(); let mut has_error
                        = false; if headline.is_empty() { create_note_headline_error_signal
                        .set("Headline cannot be empty!".to_string()); has_error = true; } if content
                        .is_empty() { create_note_content_error_signal.set("Content cannot be empty!"
                        .to_string()); has_error = true; } if has_error { create_note_error_signal
                        .set("-".to_string()); return; } let result = add_team_note(project_id, team_id,
                        headline.clone(), content.clone()). await; if let Err(e) = result {
                        console::error_1(& format!("Error creating note: {e}") .into());
                        creating_error_signal.set("Error creating note!".to_string()); } else { let
                        new_note = NoteData { id : Uuid::new_v4(), headline, content, created :
                        Utc::now(), }; sorted_note_list_signal.write().insert(0, new_note);
                        create_note_headline_signal.set(String::new()); create_note_content_signal
                        .set(String::new()); create_note_headline_error_signal.set(String::new());
                        create_note_content_error_signal.set(String::new()); create_note_error_signal
                        .set(String::new()); creating_error_signal.set(String::new()); } }
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

// ─────────────────────────────────────────────
//  Share dialog
// ─────────────────────────────────────────────

#[component]
fn ShareDialog(
    team_dialog_signal: Signal<PopUpWindow>,
    project_id: Uuid,
    share_config_option: Option<ShareTeamConfig>,
    number_of_teams: usize,
) -> Element {
    let share_config = share_config_option.clone().unwrap_or_default();
    let is_share_config_some = share_config_option.is_some();

    let mut share_config_signal = use_signal(|| share_config.clone());

    let mut share_config_date = use_signal(|| {
        share_config_signal
            .read()
            .registration_deadline
            .map(|dt| dt.with_timezone(&Local).date_naive())
    });
    let mut share_config_time = use_signal(|| {
        share_config_signal
            .read()
            .registration_deadline
            .map(|dt| dt.with_timezone(&Local).time())
    });

    let share_url: Signal<String> = use_signal(|| {
        let base_url = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_else(|| "unknown".to_string());
        format!("{base_url}/cook-and-run/{project_id}/share")
    });

    let deadline_passed = share_config_signal
        .read()
        .registration_deadline
        .map_or(false, |deadline| deadline < Utc::now());

    let mut current_config_signal = use_signal(|| is_share_config_some && !deadline_passed);

    let max_teams_text = match share_config_signal.read().max_teams {
        Some(0) | None => "∞".to_string(),
        Some(max) => max.to_string(),
    };

    let max_teams_reached = match share_config_signal.read().max_teams {
        Some(0) | None => false,
        Some(max) => number_of_teams >= max as usize,
    };

    let is_active_signal = use_signal(|| !max_teams_reached);

    let qr_data_uri = use_memo(move || {
        let code = match QrCode::new(share_url.read().as_bytes()) {
            Ok(c) => c,
            Err(_) => return String::new(),
        };
        let svg_xml = code
            .render::<svg::Color>()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();
        let encoded = general_purpose::STANDARD.encode(svg_xml);
        format!("data:image/svg+xml;base64,{encoded}")
    });

    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let save_update_share_config: AsyncAction = async_action!({
        if let Some(date) = *share_config_date.read() {
            let time = share_config_time
                .read()
                .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap_or_default());
            let local_datetime = date.and_time(time);

            let utc_datetime = local_datetime
                .and_local_timezone(chrono::Local)
                .single()
                .map(|local_dt| local_dt.with_timezone(&chrono::Utc));

            share_config_signal.write().registration_deadline = utc_datetime;
        } else {
            share_config_signal.write().registration_deadline = None;
        }

        let config = share_config_signal.read().clone();
        let create_config = config.to_create();
        let mut storage = storage_signal.write();

        let result = if !is_share_config_some {
            storage
                .create_cook_and_run_share_config(project_id, &create_config)
                .await
        } else {
            storage
                .update_cook_and_run_share_config(project_id, &create_config)
                .await
        };

        if let Err(e) = result {
            console::error_1(&format!("Error saving share config: {e}").into());
        } else {
            team_dialog_signal.set(PopUpWindow::Share(Some(config)));
        }
    });

    let mut toggle_required_field = move |field: storage::RequiredField| {
        let mut config = share_config_signal.write();
        if let Some(pos) = config.required_fields.iter().position(|f| *f == field) {
            config.required_fields.remove(pos);
        } else {
            config.required_fields.push(field);
        }
    };

    rsx! {
        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen justify-center items-center bg-black/20 z-50 p-4",
            div { class: "relative bg-white rounded-2xl border border-amber-100 shadow-xl w-full max-w-4xl max-h-[90vh] flex flex-col overflow-hidden",
                // Stabile Header-Leiste
                div { class: "px-6 py-4 bg-amber-50/70 border-b border-amber-100 flex items-center justify-between gap-2.5 shrink-0 relative",
                    div { class: "flex items-center gap-2.5",
                        div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                        span { class: "text-base font-semibold text-zinc-800", "Share Configuration" }
                    }
                    CloseButton {
                        onclick: move |_| {
                            team_dialog_signal.set(PopUpWindow::None);
                        },
                    }
                }

                // Scrollbarer Inhaltsbereich
                div { class: "px-6 py-5 overflow-y-auto flex-1",
                    div { class: "flex border-b border-amber-100 mb-5",
                        button {
                            r#type: "button",
                            onclick: move |_| {
                                current_config_signal.set(true);
                            },
                            disabled: share_config_option.is_none() || deadline_passed,
                            class: if share_config_option.is_none() || deadline_passed { "px-4 py-2 text-sm font-medium text-zinc-300 cursor-not-allowed opacity-50" } else { tab_cls(*current_config_signal.read()) },
                            "Info"
                        }
                        button {
                            r#type: "button",
                            class: tab_cls(!*current_config_signal.read()),
                            onclick: move |_| {
                                current_config_signal.set(false);
                            },
                            "Config"
                        }
                    }

                    if *current_config_signal.read() {
                        div { class: "space-y-4",
                            div { class: "grid grid-cols-2 gap-4 mb-4",
                                div { class: if max_teams_reached { "p-4 rounded-xl border border-red-200 bg-red-50" } else { "p-4 rounded-xl border border-amber-100 bg-amber-50/50" },
                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-zinc-400 mb-1",
                                        "Teams Registered"
                                    }
                                    p { class: "text-2xl font-bold text-[#C66741]",
                                        "{number_of_teams}"
                                    }
                                }
                                div { class: if max_teams_reached { "p-4 rounded-xl border border-red-200 bg-red-50" } else { "p-4 rounded-xl border border-amber-100 bg-amber-50/50" },
                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-zinc-400 mb-1",
                                        "Teams Allowed"
                                    }
                                    p { class: "text-2xl font-bold text-[#C66741]",
                                        "{max_teams_text}"
                                    }
                                }
                            }

                            if max_teams_reached {
                                div { class: "rounded-xl border border-amber-300 bg-amber-50 px-4 py-3 mb-4",
                                    p { class: "text-sm font-semibold text-amber-700",
                                        "Maximum team limit reached – no more teams can join via the share link."
                                    }
                                }
                            }

                            div { class: if *is_active_signal.read() { "flex flex-col space-y-4" } else { "flex flex-col space-y-4 opacity-50 pointer-events-none" },
                                div { class: "flex flex-col items-center p-4 rounded-xl border border-amber-100 bg-amber-50/30",
                                    label { class: "{LBL} mb-3", "QR Code" }
                                    div { class: "bg-white p-3 rounded-xl border border-amber-100",
                                        img {
                                            src: "{qr_data_uri}",
                                            alt: "QR Code",
                                            class: "w-44 h-44",
                                        }
                                    }
                                    button {
                                        class: "mt-3 px-4 py-2 text-sm font-medium rounded-xl bg-[#D67229] hover:bg-[#C66741] text-white transition-colors duration-150",
                                        disabled: !*is_active_signal.read(),
                                        onclick: move |_| {
                                            let qr_data = qr_data_uri.read().clone();
                                            if !qr_data.is_empty() {
                                                if let Some(window) = web_sys::window() {
                                                    let document = window.document().unwrap();
                                                    if let Ok(link) = document.create_element("a") {
                                                        let _ = link.set_attribute("href", &qr_data);
                                                        let _ = link.set_attribute("download", "qr-code.svg");
                                                        if let Some(body) = document.body() {
                                                            let _ = body.append_child(&link);
                                                            if let Some(elem) = link.dyn_ref::<web_sys::HtmlElement>() {
                                                                elem.click();
                                                            }
                                                            let _ = body.remove_child(&link);
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        "Download QR Code"
                                    }
                                }

                                div { class: "flex flex-col",
                                    label { class: "{LBL}", "Share Link" }
                                    div { class: "flex gap-2 mt-1",
                                        input {
                                            r#type: "text",
                                            readonly: true,
                                            disabled: !*is_active_signal.read(),
                                            value: "{share_url}",
                                            class: "{NATIVE_INPUT} flex-1",
                                        }
                                        button {
                                            class: "px-4 py-2 text-sm font-medium rounded-xl bg-[#D67229] hover:bg-[#C66741] text-white transition-colors duration-150",
                                            disabled: !*is_active_signal.read(),
                                            onclick: move |_| {},
                                            "Copy"
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        div { class: "flex flex-col space-y-5",
                            div {
                                label { class: "{LBL}", "Invitation Text" }
                                TextArea {
                                    placeholder: Some("Enter an invitation text...".to_string()),
                                    value: share_config_signal.read().invite_text.clone(),
                                    is_error: false,
                                    oninput: move |data: Event<FormData>| {
                                        share_config_signal.write().invite_text =
                                            data.value().to_string();
                                    },
                                }
                            }

                            div { class: "grid grid-cols-2 gap-3",
                                label {
                                    class: "flex items-center gap-2.5 px-3 py-2.5 rounded-xl border border-amber-100 bg-amber-50/30 hover:bg-amber-50/60 transition-colors cursor-pointer group",
                                    title: "Teams must be logged in to join",
                                    input {
                                        r#type: "checkbox",
                                        checked: share_config_signal.read().needs_login,
                                        class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                        onclick: move |_| {
                                            let current = share_config_signal.read().needs_login;
                                            share_config_signal.write().needs_login = !current;
                                        },
                                    }
                                    span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                                        "Login required"
                                    }
                                }
                                label {
                                    class: "flex items-center gap-2.5 px-3 py-2.5 rounded-xl border border-amber-100 bg-amber-50/30 hover:bg-amber-50/60 transition-colors cursor-pointer group",
                                    title: "New teams require verification before participation",
                                    input {
                                        r#type: "checkbox",
                                        checked: share_config_signal.read().default_needs_check,
                                        class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                        onclick: move |_| {
                                            let current = share_config_signal.read().default_needs_check;
                                            share_config_signal.write().default_needs_check = !current;
                                        },
                                    }
                                    span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                                        "Verification required"
                                    }
                                }
                            }

                            div { class: "rounded-xl border border-amber-100 bg-amber-50/30 p-4",
                                label { class: "{LBL} mb-3", "Required Fields" }
                                div { class: "grid grid-cols-2 gap-2",
                                    label {
                                        class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams must provide an email address",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Mail),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| toggle_required_field(storage::RequiredField::Mail),
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                                            "Email"
                                        }
                                    }

                                    label {
                                        class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams must provide a phone number",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Phone),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| toggle_required_field(storage::RequiredField::Phone),
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                                            "Phone"
                                        }
                                    }

                                    label {
                                        class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams must specify number of members",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal
                                                .read()
                                                .required_fields
                                                .contains(&storage::RequiredField::Members),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| toggle_required_field(storage::RequiredField::Members),
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                                            "Number of Members"
                                        }
                                    }

                                    label {
                                        class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams can provide dietary requirements",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Diets),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| toggle_required_field(storage::RequiredField::Diets),
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors",
                                            "Dietary Requirements"
                                        }
                                    }
                                }
                            }

                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "cursor-help group relative",
                                    label { class: "{LBL}", "Max Teams" }
                                    InputNumber {
                                        placeholder: Some("0 = unlimited".to_string()),
                                        value: share_config_signal
                                            .read()
                                            .max_teams
                                            .map_or_else(|| "0".to_string(), |v| v.to_string()),
                                        is_error: false,
                                        oninput: move |_e: Event<FormData>| {
                                            let input_value = _e.value().trim().to_string();
                                            if input_value.is_empty() || input_value == "0" {
                                                share_config_signal.write().max_teams = None;
                                            } else if let Ok(num) = input_value.parse::<u32>() {
                                                share_config_signal.write().max_teams = Some(num);
                                            }
                                        },
                                    }
                                }
                                div { class: "cursor-help group relative",
                                    label { class: "{LBL}", "Valid Until" }
                                    div { class: "flex gap-2",
                                        input {
                                            r#type: "date",
                                            class: "{NATIVE_INPUT} flex-1",
                                            value: share_config_date.read().as_ref().map(|v| v.to_string()).unwrap_or_default(),
                                            onchange: move |e: Event<FormData>| {
                                                let date_str = e.value();
                                                if let Ok(date) = date_str.parse::<chrono::NaiveDate>() {
                                                    share_config_date.set(Some(date));
                                                } else {
                                                    share_config_date.set(None);
                                                }
                                            },
                                        }
                                        input {
                                            r#type: "time",
                                            class: "{NATIVE_INPUT} flex-1",
                                            value: share_config_time.read().as_ref().map(|v| v.to_string()).unwrap_or_default(),
                                            onchange: move |e: Event<FormData>| {
                                                let time_str = e.value();
                                                if let Ok(time) = time_str.parse::<chrono::NaiveTime>() {
                                                    share_config_time.set(Some(time));
                                                } else {
                                                    share_config_time.set(None);
                                                }
                                            },
                                        }
                                    }
                                    if deadline_passed {
                                        div { class: "mt-2 rounded-xl border border-amber-300 bg-amber-50 px-3 py-2",
                                            p { class: "text-xs font-semibold text-amber-700",
                                                "The registration deadline has passed."
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "flex justify-center mt-2 pt-2",
                                ConfirmButton {
                                    text: if share_config_option.is_none() { "Create & Activate".to_string() } else { "Update".to_string() },
                                    action: save_update_share_config.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ─────────────────────────────────────────────
//  Validation helpers
// ─────────────────────────────────────────────

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
    if team_name_signal.read().trim().is_empty() {
        team_name_error_signal.set("Team name cannot be empty!".to_string());
        false
    } else {
        team_name_error_signal.set(String::new());
        true
    }
}

fn check_team_email(
    team_email_signal: Signal<String>,
    mut team_email_error_signal: Signal<String>,
) -> bool {
    let team_email = team_email_signal.read();
    let trimmed = team_email.trim();
    if trimmed.is_empty() {
        team_email_error_signal.set(String::new());
        true
    } else if !trimmed.contains('@') || !trimmed.contains('.') {
        team_email_error_signal.set("Please enter a valid email address!".to_string());
        false
    } else {
        team_email_error_signal.set(String::new());
        true
    }
}

fn check_team_tel(
    _team_tel_signal: Signal<String>,
    mut team_tel_error_signal: Signal<String>,
) -> bool {
    team_tel_error_signal.set(String::new());
    true
}

fn check_members(members_signal: Signal<String>, mut members_error_signal: Signal<String>) -> bool {
    let members = members_signal.read();
    let trimmed = members.trim();
    if trimmed.is_empty() {
        members_error_signal.set(String::new());
        true
    } else {
        match trimmed.parse::<u32>() {
            Ok(0) => {
                members_error_signal.set("Number of Members must be greater than 0!".to_string());
                false
            }
            Ok(_) => {
                members_error_signal.set(String::new());
                true
            }
            Err(_) => {
                members_error_signal.set("Please enter a valid number!".to_string());
                false
            }
        }
    }
}