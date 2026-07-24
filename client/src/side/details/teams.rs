use async_std::task::sleep;
use std::time::Duration;

use crate::side::details::address::{Address, AddressParam};
use crate::side::details::{ErrorPage, LoadingPage};
use crate::side::AsyncAction;
use crate::side::{AddressSVG, DeleteButtonProps, Headline1, Headline2, InputPhoneNumber};
use crate::storage::{
    AddressData, NoteCreate, NoteData, ShareTeamConfig, StorageManager, TeamCreate, TeamData,
    TeamUpdate,
};
use crate::{async_action, storage};
use base64::engine::general_purpose;
use base64::Engine;
use chrono::{Local, TimeZone, Utc};
use dioxus::prelude::*;
use qrcode::render::svg;
use qrcode::QrCode;
use uuid::Uuid;
use web_sys::console;
use web_sys::wasm_bindgen::JsCast;

use crate::side::{CloseButton, ConfirmButton, Input, InputError, InputMultirow, InputNumber};

// ─────────────────────────────────────────────
//  Shared design tokens
// ─────────────────────────────────────────────

const LBL: &str =
    "block text-[11px] font-semibold tracking-[0.12em] uppercase text-amber-700/70 mb-1.5";

const NATIVE_INPUT: &str = "w-full px-3 py-2 rounded-xl border border-amber-200 bg-amber-50/40 \
     text-sm text-zinc-800 placeholder-zinc-400 \
     focus:outline-none focus:ring-2 focus:ring-amber-400/40 focus:border-amber-400 \
     transition-colors duration-150";

// ─────────────────────────────────────────────
//  CSS: Keyframe-Animation für den grünen Glow
// ─────────────────────────────────────────────

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

// Tab class helper – avoids duplicating the ternary in every button
fn tab_cls(active: bool) -> &'static str {
    if active {
        "px-4 py-2 text-sm font-semibold text-[#C66741] border-b-2 border-[#C66741] -mb-px"
    } else {
        "px-4 py-2 text-sm font-medium text-zinc-500 hover:text-[#C66741] transition-colors"
    }
}

// ─────────────────────────────────────────────
//  Helpers (unchanged logic)
// ─────────────────────────────────────────────

fn map_string(value: String) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn map_u8(value: String) -> Option<u8> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        value.parse::<u8>().map_or_else(|_| None, |v| Some(v))
    }
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
        needs_check: needs_check,
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
        let storage = storage.clone();
        async move {
            let storage = storage.read().clone();
            let team_list = storage
                .select_cook_and_run_team_list(cook_and_run_id)
                .await?;
            let meta = storage.select_cook_and_run_meta(cook_and_run_id).await?;
            Ok((team_list, meta.is_in_cloud))
        }
    });

    match &*team_list.read_unchecked() {
        None => rsx!(LoadingPage {}),
        Some(Err(e)) => rsx!(ErrorPage {
            error_text:
                "Could not load project. You may need to log in or the servers may be offline."
                    .to_string(),
            error_details: e.clone(),
        }),
        Some(Ok(team_list)) => rsx!(TeamsContent {
            cook_and_run_id,
            team_list: team_list.0.clone(),
            is_online: team_list.1,
        }),
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

    // Hover state for share button and add-team tile – kept as inline style
    // so Tailwind's JIT doesn't need to know about these at compile time.
    let mut share_hovered = use_signal(|| false);
    let mut add_team_hovered = use_signal(|| false);

    let storage = use_context::<Signal<StorageManager>>();
    let share_config: Resource<Result<ShareConfigState, String>> =
        use_resource(move || async move {
            if is_online == false {
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

    // Share icon SVG (reused in multiple match arms)
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
            line { x1: "8.59", y1: "13.51", x2: "15.42", y2: "17.49" }
            line { x1: "15.41", y1: "6.51", x2: "8.59", y2: "10.49" }
        }
    };

    // Inline styles for share button hover (same pattern as original)
    let share_btn_style = format!(
        "display:flex;align-items:center;justify-content:center;\
         width:36px;height:36px;border-radius:10px;\
         border:none;cursor:pointer;\
         background-color:{};color:white;\
         transition:background-color 0.15s ease,box-shadow 0.15s ease;{}",
        if share_hovered() {
            "#C66741"
        } else {
            "#D67229"
        },
        if share_hovered() {
            "box-shadow:0 4px 14px rgba(198,103,65,0.45);"
        } else {
            "box-shadow:0 2px 8px rgba(214,114,41,0.3);"
        }
    );

    // Inline styles for add-team tile hover
    let add_team_style = format!(
        "border:2px dashed {};border-radius:16px;padding:20px;\
         display:flex;flex-direction:column;align-items:center;justify-content:center;gap:10px;\
         cursor:pointer;color:{};\
         background-color:{};\
         transition:all 0.15s ease;",
        if add_team_hovered() {
            "#D67229"
        } else {
            "rgba(214,114,41,0.35)"
        },
        if add_team_hovered() {
            "#C66741"
        } else {
            "rgba(214,114,41,0.5)"
        },
        if add_team_hovered() {
            "rgba(253,246,236,0.8)"
        } else {
            "rgba(253,246,236,0.3)"
        },
    );

    let config = match share_config.read_unchecked().clone() {
        // Loading skeleton
        None => rsx! {
            div { class: "w-9 h-9 rounded-xl bg-amber-100/60 animate-pulse" }
        },
        // Error
        Some(Err(e)) => rsx! {
            div {
                class: "flex items-center justify-center w-9 h-9 rounded-xl \
                        bg-red-50 border border-red-200 text-red-400",
                title: "Error loading share config: {e}",
                svg {
                    class: "w-4 h-4",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    line { x1: "18", y1: "6", x2: "6", y2: "18" }
                    line { x1: "6", y1: "6", x2: "18", y2: "18" }
                }
            }
        },
        // Offline – cloud only
        Some(Ok(ShareConfigState::Offline)) => rsx! {
            div {
                class: "flex items-center justify-center w-9 h-9 rounded-xl \
                        bg-zinc-100 border border-zinc-200 text-zinc-300 cursor-not-allowed",
                title: "Sharing is only available when the project is stored in the cloud.",
                button {
                    class: "flex items-center justify-center w-full h-full",
                    disabled: true,
                    {share_icon_svg}
                }
            }
        },
        // Active – share config loaded
        Some(Ok(ShareConfigState::Loaded(config))) => rsx! {
            button {
                style: "{share_btn_style}",
                onmouseenter: move |_| share_hovered.set(true),
                onmouseleave: move |_| share_hovered.set(false),
                onclick: move |_| {
                    team_dialog_signal.set(PopUpWindow::Share(Some(config.clone())));
                },
                {share_icon_svg}
            }
        },
        // Active – no share config yet
        Some(Ok(ShareConfigState::None)) => rsx! {
            button {
                style: "{share_btn_style}",
                onmouseenter: move |_| share_hovered.set(true),
                onmouseleave: move |_| share_hovered.set(false),
                onclick: move |_| {
                    team_dialog_signal.set(PopUpWindow::Share(None));
                },
                {share_icon_svg}
            }
        },
    };

    rsx! {
        section { class: "px-8 py-6 space-y-8 w-full",

            // ── Page header ───────────────────────────────────────
            div { class: "pb-5 flex items-end justify-between",
                div {
                    Headline1 { headline: "Teams" }
                }
                {config}
            }

            // ── Team grid ─────────────────────────────────────────
            div { class: "grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 \
                          max-h-[calc(100vh-16rem)] overflow-y-auto pr-1",

                {
                    team_list.iter().map(|team| {
                        let team_data = team.clone();
                        let needs_check = team.needs_check;
                        let card_cls = if needs_check {
                            "relative bg-white rounded-2xl border border-amber-400 \
                             shadow-sm hover:shadow-md transition-all duration-150 \
                             cursor-pointer overflow-hidden"
                        } else {
                            "relative bg-white rounded-2xl border border-amber-100 \
                             shadow-sm hover:shadow-md transition-all duration-150 \
                             cursor-pointer overflow-hidden"
                        };
                        rsx! {
                            a {
                                onclick: move |_| {
                                    team_dialog_signal.set(PopUpWindow::EditTeam(team_data.clone()));
                                },
                                class: "{card_cls}",
                                {TeamCard(team.clone())}
                            }
                        }
                    })
                }

                // ── Add team tile ─────────────────────────────────
                a {
                    style: "{add_team_style}",
                    onmouseenter: move |_| add_team_hovered.set(true),
                    onmouseleave: move |_| add_team_hovered.set(false),
                    onclick: move |_| {
                        team_dialog_signal.set(PopUpWindow::AddTeam);
                    },
                    div {
                        style: "width:28px;height:28px;border-radius:50%;border:2px solid currentColor;\
                                display:flex;align-items:center;justify-content:center;\
                                font-size:18px;font-weight:700;line-height:1;",
                        "+"
                    }
                    span { style: "font-size:13px;font-weight:600;letter-spacing:0.04em;", "Add team" }
                }
            }
        }

        // ── Dialogs ───────────────────────────────────────────────
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

#[component]
fn TeamCard(props: TeamData) -> Element {
    let needs_check = props.needs_check;
    let style_0 = if props.needs_check {
        "bg-amber-100/80 border-amber-300"
    } else {
        "bg-amber-50/70 border-amber-100"
    };
    let style_1 = if props.needs_check {
        "bg-amber-500"
    } else {
        "bg-amber-400/70"
    };
    rsx! {
        div {
            // Amber header stripe
            div { class: "px-4 py-2.5 border-b flex items-center gap-2 {style_0}",
                div { class: "w-1.5 h-4 rounded-full shrink-0 {style_1}" }
                span { class: "text-sm font-semibold text-zinc-800 truncate", "{props.name}" }
                if needs_check {
                    span { class: "ml-auto text-[10px] font-bold px-1.5 py-0.5 \
                                   rounded-full bg-amber-500 text-white tracking-wide shrink-0",
                        "REVIEW"
                    }
                }
            }
            // Card body
            div { class: "px-4 py-3",
                div { class: "flex items-center gap-1.5",
                    AddressSVG {}
                    p { class: "text-xs text-zinc-500 truncate", "{props.address.address}" }
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
        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen \
                      justify-center items-center bg-black/20 z-50",
            div { class: "relative bg-white rounded-2xl border border-amber-100 \
                          shadow-xl w-224 overflow-y-auto max-h-[90vh]",

                // Dialog header
                div { class: "px-6 py-4 bg-amber-50/70 border-b border-amber-100 \
                              flex items-center gap-2.5",
                    div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                    span { class: "text-base font-semibold text-zinc-800", "Add Team" }
                }

                // Close button
                CloseButton {
                    onclick: move |_| { team_dialog_signal.set(PopUpWindow::None); },
                }

                div { class: "px-6 py-5",
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
                                    let result = add_team(
                                        project_id,
                                        team_name_signal.read().to_string(),
                                        diets_signal.read().clone(),
                                        team_email_signal.read().clone(),
                                        team_tel_signal.read().clone(),
                                        members_signal.read().clone(),
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
    let team_name_signal = use_signal(|| team_data.name);
    let team_name_error_signal = use_signal(|| "".to_string());
    let team_email_signal = use_signal(|| team_data.mail.map_or_else(|| "".to_string(), |v| v));
    let team_email_error_signal = use_signal(|| "".to_string());
    let team_tel_signal = use_signal(|| team_data.phone.map_or_else(|| "".to_string(), |v| v));
    let team_tel_error_signal = use_signal(|| "".to_string());
    let members_signal = use_signal(|| {
        team_data
            .members
            .map_or_else(|| "".to_string(), |v| v.to_string())
    });
    let members_error_signal = use_signal(|| "".to_string());
    let diets_signal = use_signal(|| team_data.diets.map_or_else(|| "".to_string(), |v| v));
    let mut needs_check_signal = use_signal(|| team_data.needs_check);
    let error_signal = use_signal(|| "".to_string());
    let mut is_edit_team_signal = use_signal(|| true);
    let address_param = AddressParam::new(&team_data.address);

    // true  → grüner Glow für 2 s, dann Dialog schließen
    let mut save_success_signal = use_signal(|| false);

    // true  → Felder wurden verändert → Update-Button aktiv
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
            address_param,
        );
    });

    let delete_button = DeleteButtonProps::new(
        async_action!({
            let result = delete_team(project_id, team_data.id).await;
            if let Err(e) = result {
                console::error_1(&format!("Error deleting team: {}", e).into());
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

    // Dialog-Rahmen: im Erfolgsfall grüner Glow
    let dialog_class = if is_success {
        "relative bg-white rounded-2xl border shadow-xl w-224 overflow-y-auto max-h-[90vh] save-glow-dialog"
    } else {
        "relative bg-white rounded-2xl border border-amber-100 shadow-xl w-224 overflow-y-auto max-h-[90vh]"
    };
    let header_class = if is_success {
        "px-6 py-4 border-b flex items-center gap-2.5 save-glow-header"
    } else {
        "px-6 py-4 bg-amber-50/70 border-b border-amber-100 flex items-center gap-2.5"
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

        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen \
                      justify-center items-center bg-black/20 z-50",
            div { class: "{dialog_class}",

                // Dialog header
                div { class: "{header_class}",
                    div { class: "{accent_class}" }
                    span { class: "{title_class}", "Edit Team" }
                }

                // Close button
                CloseButton {
                    onclick: move |_| { team_dialog_signal.set(PopUpWindow::None); },
                }

                div { class: "px-6 py-5",

                    // Tab bar + needs-check + delete
                    div { class: "flex items-center justify-between border-b border-amber-100 mb-5",
                        div { class: "flex",
                            button {
                                r#type: "button",
                                class: tab_cls(*is_edit_team_signal.read()),
                                onclick: move |_| { is_edit_team_signal.set(true); },
                                "Team Data"
                            }
                            button {
                                r#type: "button",
                                class: tab_cls(!*is_edit_team_signal.read()),
                                onclick: move |_| { is_edit_team_signal.set(false); },
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
                                        team_data.needs_check = new_value;
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
                            address_param,
                            on_change: move |_| { has_unsaved_changes.set(true); },
                        }

                        div { class: "flex justify-center mt-5",
                            div { class: "{update_btn_wrapper_class}",
                                ConfirmButton {
                                text: "Update Team".to_string(),
                                error_signal: error_signal.clone(),
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
                                        let result = update_team(
                                            project_id,
                                            team_data.id,
                                            team_name_signal.read().to_string(),
                                            diets_signal.read().clone(),
                                            team_email_signal.read().clone(),
                                            team_tel_signal.read().clone(),
                                            members_signal.read().clone(),
                                            address_param
                                                .get_address_data()
                                                .expect("Expext no errors when getting address_data!"),
                                            needs_check_signal.read().clone(),
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
                                            // Erfolg: Dialog grün aufblinken lassen,
                                            // nach 2 s automatisch schließen
                                            save_success_signal.set(true);
                                            spawn(async move {
                                                sleep(Duration::from_millis(2000)).await;
                                                team_dialog_signal.set(PopUpWindow::None);
                                            });
                                        }
                                    }
                                ),
                            }
                            }  // end update_btn_wrapper_class div
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
    // Callback: wird bei jeder Feldänderung aufgerufen (z.B. für has_unsaved_changes)
    on_change: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-col md:flex-row gap-6",

            // Left: fields
            div { class: "flex-1 space-y-3 md:border-r border-amber-100 md:pr-5",

                div {
                    label { class: "{LBL}", "Team Name" }
                    Input {
                        place_holer: Some("e.g. The Chili Chasers".to_string()),
                        is_error: !team_name_error_signal.read().is_empty(),
                        value: team_name_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let team_name = e.value();
                            team_name_signal.set(team_name.clone());
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
                            team_email_signal.set(team_email.clone());
                            check_team_email(team_email_signal, team_email_error_signal);
                            on_change.call(());
                        },
                    }
                    InputError { error: team_email_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Phone Number" }
                    InputPhoneNumber {
                        place_holer: Some("e.g. +49 1234 56789".to_string()),
                        is_error: !team_tel_error_signal.read().is_empty(),
                        value: team_tel_signal.clone(),
                        oninput: move |e: Event<FormData>| {
                            let team_tel = e.value();
                            team_tel_signal.set(team_tel.clone());
                            check_team_tel(team_tel_signal, team_tel_error_signal);
                            on_change.call(());
                        },
                    }
                    InputError { error: team_tel_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Number of Members" }
                    InputNumber {
                        place_holer: Some("e.g. 2".to_string()),
                        value: members_signal.clone(),
                        is_error: !members_error_signal.read().is_empty(),
                        oninput: move |e: Event<FormData>| {
                            let members = e.value();
                            members_signal.set(members.clone());
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
                                let diets = e.value();
                                diets_signal.set(diets.clone());
                                on_change.call(());
                            },
                        }
                    }
                }
            }

            // Right: address
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
    let mut create_note_headline_signal = use_signal(|| "".to_string());
    let mut create_note_headline_error_signal = use_signal(|| "".to_string());
    let mut create_note_content_signal = use_signal(|| "".to_string());
    let mut create_note_content_error_signal = use_signal(|| "".to_string());
    let mut create_note_error_signal = use_signal(|| "".to_string());
    let mut creating_error_signal = use_signal(|| "".to_string());

    let mut sorted_note_list = note_data_list.clone();
    sorted_note_list.sort_by(|a, b| b.created.cmp(&a.created));
    let mut sorted_note_list_signal = use_signal(|| sorted_note_list);

    rsx! {
        div { class: "flex flex-col md:flex-row gap-6",

            // Left: create note
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
                            } else if create_note_content_error_signal.read().is_empty() {
                                create_note_headline_error_signal.set("".to_string());
                                create_note_error_signal.set("".to_string());
                            } else {
                                create_note_headline_error_signal.set("".to_string());
                            }
                        },
                    }
                    InputError { error: create_note_headline_error_signal.read() }
                }

                div {
                    label { class: "{LBL}", "Content" }
                    InputMultirow {
                        place_holer: Some("e.g. Participation fee is partially paid!".to_string()),
                        value: create_note_content_signal.clone(),
                        is_error: !create_note_content_error_signal.read().is_empty(),
                        oninput: move |e: Event<FormData>| {
                            let content = e.value();
                            create_note_content_signal.set(content.clone());
                            if content.is_empty() {
                                create_note_content_error_signal
                                    .set("content cannot be empty!".to_string());
                                create_note_error_signal.set("-".to_string());
                            } else if create_note_headline_error_signal.read().is_empty() {
                                create_note_content_error_signal.set("".to_string());
                                create_note_error_signal.set("".to_string());
                            } else {
                                create_note_content_error_signal.set("".to_string());
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
                            if create_note_headline_signal.read().is_empty()
                                && create_note_content_signal.read().is_empty()
                            {
                                create_note_headline_error_signal
                                    .set("Headline cannot be empty!".to_string());
                                create_note_content_error_signal
                                    .set("content cannot be empty!".to_string());
                                create_note_error_signal.set("-".to_string());
                                return;
                            } else if create_note_headline_signal.read().is_empty() {
                                create_note_headline_error_signal
                                    .set("Headline cannot be empty!".to_string());
                                create_note_error_signal.set("-".to_string());
                                return;
                            } else if create_note_content_signal.read().is_empty() {
                                create_note_content_error_signal
                                    .set("content cannot be empty!".to_string());
                                create_note_error_signal.set("-".to_string());
                                return;
                            }
                            let result = add_team_note(
                                project_id,
                                team_id,
                                create_note_headline_signal.read().trim().to_string(),
                                create_note_content_signal.read().trim().to_string(),
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
                                sorted_note_list_signal.set({
                                    let mut note_list = vec![NoteData {
                                        id: Uuid::new_v4(),
                                        headline: create_note_headline_signal
                                            .read()
                                            .trim()
                                            .to_string(),
                                        content: create_note_content_signal
                                            .read()
                                            .trim()
                                            .to_string(),
                                        created: Utc::now().naive_utc(),
                                    }];
                                    note_list.extend(sorted_note_list_signal.read().clone());
                                    note_list
                                });
                                create_note_headline_signal.set("".to_string());
                                create_note_content_signal.set("".to_string());
                                create_note_headline_error_signal.set("".to_string());
                                create_note_content_error_signal.set("".to_string());
                                create_note_error_signal.set("".to_string());
                                creating_error_signal.set("".to_string());
                            }
                        }
                    ),
                }
                InputError { error: creating_error_signal.read() }
            }

            // Right: note list
            div { class: "flex-1",
                label { class: "{LBL}", "Notes" }
                div { class: "space-y-2 overflow-y-auto max-h-80 mt-1",
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
    let created = Local
        .from_local_datetime(&note_data.created)
        .single()
        .unwrap()
        .format("%Y-%m-%d %H:%M")
        .to_string();

    rsx!(
        div { class: "bg-amber-50/50 rounded-xl border border-amber-100 px-4 py-3",
            div { class: "flex justify-between items-baseline gap-2 mb-1",
                span { class: "text-sm font-semibold text-zinc-800 truncate",
                    "{note_data.headline}"
                }
                span { class: "text-[11px] text-zinc-400 shrink-0", "{created}" }
            }
            p { class: "text-xs text-zinc-600 leading-relaxed", "{note_data.content}" }
        }
    )
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
    let share_config = match share_config_option.clone() {
        Some(config) => config,
        None => ShareTeamConfig::default(),
    };

    let mut share_config_signal = use_signal(|| share_config.clone());

    let mut share_config_date = use_signal(|| {
        share_config_signal
            .read()
            .registration_deadline
            .map(|dt| dt.date())
    });
    let mut share_config_time = use_signal(|| {
        share_config_signal
            .read()
            .registration_deadline
            .map(|dt| dt.time())
    });

    let share_url: Signal<String> = use_signal(|| {
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();
        let base_url = location.origin().unwrap_or_else(|_| "unknown".to_string());
        format!("{}/cook-and-run/{}/share", base_url, project_id)
    });

    let deadline_passed = share_config_signal
        .read()
        .registration_deadline
        .map_or(false, |deadline| deadline < Utc::now().naive_utc());

    let is_share_config_some = share_config_option.is_some();
    let mut current_config_signal = use_signal(|| (is_share_config_some && !deadline_passed));

    let max_teams_text = share_config_signal
        .read()
        .max_teams
        .map(|max_teams| {
            if max_teams == 0 {
                "∞".to_string()
            } else {
                max_teams.to_string()
            }
        })
        .unwrap_or_else(|| "∞".to_string());

    let max_teams_reached = number_of_teams
        >= share_config_signal
            .read()
            .max_teams
            .map(|max_teams| {
                if max_teams == 0 {
                    usize::MAX
                } else {
                    max_teams as usize
                }
            })
            .unwrap_or_else(|| usize::MAX);

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
        format!("data:image/svg+xml;base64,{}", encoded)
    });

    let mut storage_signal = use_context::<Signal<StorageManager>>();
    let save_update_share_config: AsyncAction = async_action!({
        if let Some(date) = share_config_date.read().clone() {
            let time = share_config_time
                .read()
                .unwrap_or(chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap());
            let local_datetime = date.and_time(time);

            let utc_datetime = local_datetime
                .and_local_timezone(chrono::Local)
                .single()
                .map(|local_dt| local_dt.with_timezone(&chrono::Utc).naive_utc());

            share_config_signal.write().registration_deadline = utc_datetime;
        } else {
            share_config_signal.write().registration_deadline = None;
        }
        let config = share_config_signal.read().clone();
        let create_config = config.to_create();
        let mut storage = storage_signal.write().clone();
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
            console::error_1(&format!("Error saving share config: {}", e).into());
        } else {
            team_dialog_signal.set(PopUpWindow::Share(Some(config)));
        }
    });

    rsx! {
        div { class: "backdrop-blur-sm fixed inset-0 flex h-screen w-screen \
                      justify-center items-center bg-black/20 z-50",
            div { class: "relative bg-white rounded-2xl border border-amber-100 \
                          shadow-xl w-224 overflow-y-auto max-h-[90vh]",

                // Dialog header
                div { class: "px-6 py-4 bg-amber-50/70 border-b border-amber-100 \
                              flex items-center gap-2.5",
                    div { class: "w-1.5 h-5 rounded-full bg-amber-400/70" }
                    span { class: "text-base font-semibold text-zinc-800", "Share Configuration" }
                }

                // Close button
                CloseButton {
                    onclick: move |_| { team_dialog_signal.set(PopUpWindow::None); },
                }

                div { class: "px-6 py-5",

                    // Tab bar
                    div { class: "flex border-b border-amber-100 mb-5",
                        button {
                            r#type: "button",
                            onclick: move |_| { current_config_signal.set(true); },
                            disabled: share_config_option.is_none() || deadline_passed,
                            class: if share_config_option.is_none() || deadline_passed {
                                "px-4 py-2 text-sm font-medium text-zinc-300 cursor-not-allowed opacity-50"
                            } else {
                                tab_cls(*current_config_signal.read())
                            },
                            "Info"
                        }
                        button {
                            r#type: "button",
                            class: tab_cls(!*current_config_signal.read()),
                            onclick: move |_| { current_config_signal.set(false); },
                            "Config"
                        }
                    }

                    if *current_config_signal.read() {
                        // ── Info tab ──────────────────────────────
                        div { class: "space-y-4",

                            // Stat cards
                            div { class: "grid grid-cols-2 gap-4 mb-4",
                                div { class: if max_teams_reached {
                                        "p-4 rounded-xl border border-red-200 bg-red-50"
                                    } else {
                                        "p-4 rounded-xl border border-amber-100 bg-amber-50/50"
                                    },
                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-zinc-400 mb-1",
                                        "Teams Registered"
                                    }
                                    p { class: "text-2xl font-bold text-[#C66741]", "{number_of_teams}" }
                                }
                                div { class: if max_teams_reached {
                                        "p-4 rounded-xl border border-red-200 bg-red-50"
                                    } else {
                                        "p-4 rounded-xl border border-amber-100 bg-amber-50/50"
                                    },
                                    p { class: "text-[11px] font-semibold uppercase tracking-[0.12em] text-zinc-400 mb-1",
                                        "Teams Allowed"
                                    }
                                    p { class: "text-2xl font-bold text-[#C66741]", "{max_teams_text}" }
                                }
                            }

                            if max_teams_reached {
                                div { class: "rounded-xl border border-amber-300 bg-amber-50 px-4 py-3 mb-4",
                                    p { class: "text-sm font-semibold text-amber-700",
                                        "Maximum team limit reached – no more teams can join via the share link."
                                    }
                                }
                            }

                            // QR + link (dimmed when inactive)
                            div { class: if *is_active_signal.read() {
                                    "flex flex-col space-y-4"
                                } else {
                                    "flex flex-col space-y-4 opacity-50 pointer-events-none"
                                },

                                // QR code
                                div { class: "flex flex-col items-center p-4 rounded-xl \
                                              border border-amber-100 bg-amber-50/30",
                                    label { class: "{LBL} mb-3", "QR Code" }
                                    div { class: "bg-white p-3 rounded-xl border border-amber-100",
                                        img {
                                            src: "{qr_data_uri}",
                                            alt: "QR Code",
                                            class: "w-44 h-44",
                                        }
                                    }
                                    button {
                                        class: "mt-3 px-4 py-2 text-sm font-medium rounded-xl \
                                                bg-[#D67229] hover:bg-[#C66741] text-white \
                                                transition-colors duration-150",
                                        disabled: !*is_active_signal.read(),
                                        onclick: move |_| {
                                            let qr_data = qr_data_uri.read().clone();
                                            if !qr_data.is_empty() {
                                                if let Some(window) = web_sys::window() {
                                                    let document = window.document().unwrap();
                                                    let link = document.create_element("a").unwrap();
                                                    link.set_attribute("href", &qr_data).unwrap();
                                                    link.set_attribute("download", "qr-code.svg").unwrap();
                                                    document.body().unwrap().append_child(&link).unwrap();
                                                    link.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
                                                    document.body().unwrap().remove_child(&link).unwrap();
                                                }
                                            }
                                        },
                                        "Download QR Code"
                                    }
                                }

                                // Share link
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
                                            class: "px-4 py-2 text-sm font-medium rounded-xl \
                                                    bg-[#D67229] hover:bg-[#C66741] text-white \
                                                    transition-colors duration-150",
                                            disabled: !*is_active_signal.read(),
                                            onclick: move |_| {},
                                            "Copy"
                                        }
                                    }
                                }
                            }
                        }

                    } else {
                        // ── Config tab ────────────────────────────
                        div { class: "flex flex-col space-y-5",

                            // Invitation text
                            div {
                                label { class: "{LBL}", "Invitation Text" }
                                InputMultirow {
                                    place_holer: "Enter an invitation text...".to_string(),
                                    value: share_config_signal.read().invite_text.clone(),
                                    is_error: false,
                                    oninput: move |data: Event<FormData>| {
                                        share_config_signal.write().invite_text =
                                            data.value().trim().to_string();
                                    },
                                }
                            }

                            // Settings toggles
                            div { class: "grid grid-cols-2 gap-3",
                                label {
                                    class: "flex items-center gap-2.5 px-3 py-2.5 rounded-xl \
                                            border border-amber-100 bg-amber-50/30 \
                                            hover:bg-amber-50/60 transition-colors cursor-pointer group",
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
                                    class: "flex items-center gap-2.5 px-3 py-2.5 rounded-xl \
                                            border border-amber-100 bg-amber-50/30 \
                                            hover:bg-amber-50/60 transition-colors cursor-pointer group",
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

                            // Required fields
                            div { class: "rounded-xl border border-amber-100 bg-amber-50/30 p-4",
                                label { class: "{LBL} mb-3", "Required Fields" }
                                div { class: "grid grid-cols-2 gap-2",

                                    label { class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams must provide an email address",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Mail),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| {
                                                let pos = share_config_signal.read().required_fields.iter()
                                                    .position(|req| req.eq(&storage::RequiredField::Mail));
                                                match pos {
                                                    Some(index) => { share_config_signal.write().required_fields.remove(index); }
                                                    None => { share_config_signal.write().required_fields.push(storage::RequiredField::Mail); }
                                                }
                                            },
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors", "Email" }
                                    }

                                    label { class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams must provide a phone number",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Phone),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| {
                                                let pos = share_config_signal.read().required_fields.iter()
                                                    .position(|req| req.eq(&storage::RequiredField::Phone));
                                                match pos {
                                                    Some(index) => { share_config_signal.write().required_fields.remove(index); }
                                                    None => { share_config_signal.write().required_fields.push(storage::RequiredField::Phone); }
                                                }
                                            },
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors", "Phone" }
                                    }

                                    label { class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams must specify number of members",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Members),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| {
                                                let pos = share_config_signal.read().required_fields.iter()
                                                    .position(|req| req.eq(&storage::RequiredField::Members));
                                                match pos {
                                                    Some(index) => { share_config_signal.write().required_fields.remove(index); }
                                                    None => { share_config_signal.write().required_fields.push(storage::RequiredField::Members); }
                                                }
                                            },
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors", "Number of Members" }
                                    }

                                    label { class: "flex items-center gap-2 cursor-pointer group",
                                        title: "Teams can provide dietary requirements",
                                        input {
                                            r#type: "checkbox",
                                            checked: share_config_signal.read().required_fields.contains(&storage::RequiredField::Diets),
                                            class: "accent-[#D67229] w-4 h-4 rounded cursor-pointer",
                                            onclick: move |_| {
                                                let pos = share_config_signal.read().required_fields.iter()
                                                    .position(|req| req.eq(&storage::RequiredField::Diets));
                                                match pos {
                                                    Some(index) => { share_config_signal.write().required_fields.remove(index); }
                                                    None => { share_config_signal.write().required_fields.push(storage::RequiredField::Diets); }
                                                }
                                            },
                                        }
                                        span { class: "text-[13px] text-zinc-600 group-hover:text-zinc-800 transition-colors", "Dietary Requirements" }
                                    }
                                }
                            }

                            // Max teams + valid until
                            div { class: "grid grid-cols-2 gap-4",
                                div { class: "cursor-help group relative",
                                    label { class: "{LBL}", "Max Teams" }
                                    InputNumber {
                                        place_holer: Some("0 = unlimited".to_string()),
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
                                            value: share_config_date
                                                .read()
                                                .map_or_else(|| "".to_string(), |v| v.to_string()),
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
                                            value: share_config_time
                                                .read()
                                                .map_or_else(|| "".to_string(), |v| v.to_string()),
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
                                    text: if share_config_option.is_none() {
                                        "Create & Activate".to_string()
                                    } else {
                                        "Update".to_string()
                                    },
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
//  Validation helpers (signatures unchanged)
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
        team_email_error_signal.set("".to_string());
        true
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
    team_tel_error_signal.set("".to_string());
    true
}

fn check_members(members_signal: Signal<String>, mut members_error_signal: Signal<String>) -> bool {
    let members = members_signal.read();
    if members.is_empty() {
        members_error_signal.set("".to_string());
        true
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
