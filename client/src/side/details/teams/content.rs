//! Der Hauptinhalt des Teams-Tabs: Header mit "New Team"-Button und
//! Share-Icon, Such-/Sortierleiste, das Team-Grid sowie das Umschalten
//! zwischen den drei Popups (Add/Edit/Share) je nach `PopUpWindow`.

use crate::storage::{ShareTeamConfig, StorageManager, TeamData};
use crate::ui::buttons::PrimaryButton;
use crate::ui::cards::SearchFilterCard;
use crate::ui::icons::{CloseIcon, PlusIcon, ShareIcon};
use crate::ui::typography::Headline1;
use dioxus::prelude::*;
use uuid::Uuid;

use super::add_dialog::AddTeamDialog;
use super::card::TeamCard;
use super::edit_dialog::EditTeamDialog;
use super::model::{PopUpWindow, TeamSortOption};
use super::share_dialog::ShareDialog;

#[derive(Debug, Clone, PartialEq)]
enum ShareConfigState {
    Offline,
    Loaded(ShareTeamConfig),
    None,
}

#[component]
pub(crate) fn TeamsContent(
    cook_and_run_id: Uuid,
    team_list: Vec<TeamData>,
    is_online: bool,
    on_teams_changed: EventHandler<()>,
) -> Element {
    let number_of_teams = team_list.len();
    let mut team_dialog_signal: Signal<PopUpWindow> = use_signal(|| PopUpWindow::None);

    let mut search_signal = use_signal(String::new);
    let mut sort_signal = use_signal(|| TeamSortOption::NameAsc);

    let storage = use_context::<Signal<StorageManager>>();
    let mut share_config: Resource<Result<ShareConfigState, String>> =
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

    let on_share_config_changed = move |_| {
        share_config.restart();
    };

    let share_icon_svg = rsx! {
        ShareIcon {}
    };

    let share_btn_class = "flex items-center justify-center w-9 h-9 rounded-[10px] border-none cursor-pointer bg-[#D67229] hover:bg-[#C66741] text-white shadow-[0_2px_8px_rgba(214,114,41,0.3)] hover:shadow-[0_4px_14px_rgba(198,103,65,0.45)] transition-all duration-150";

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

    // "Loaded" und "None" führen beide zum selben klickbaren Button –
    // der eigentliche Config-Wert wird nicht mehr hier, sondern erst beim
    // Öffnen des Dialogs (unten im match) frisch gelesen.
    let config = match &*share_config.read() {
        None => rsx! {
            div { class: "w-9 h-9 rounded-xl bg-amber-100/60 animate-pulse" }
        },
        Some(Err(e)) => rsx! {
            div {
                class: "flex items-center justify-center w-9 h-9 rounded-xl bg-red-50 border border-red-200 text-red-400",
                title: "Error loading share config: {e}",
                CloseIcon { class: "w-4 h-4".to_string() }
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
        Some(Ok(ShareConfigState::Loaded(_))) | Some(Ok(ShareConfigState::None)) => rsx! {
            button {
                class: "{share_btn_class}",
                onclick: move |_| {
                    team_dialog_signal.set(PopUpWindow::Share);
                },
                {share_icon_svg}
            }
        },
    };

    rsx! {
        section { class: "max-w-[1800px] mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-8 space-y-6",
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
                            let team_id = team.id;
                            let card_cls = if team.needs_check {
                                "relative bg-white rounded-2xl border border-amber-400 shadow-xs hover:shadow-md transition-all duration-150 cursor-pointer overflow-hidden"
                            } else {
                                "relative bg-white rounded-2xl border border-amber-100/80 shadow-xs hover:shadow-md transition-all duration-150 cursor-pointer overflow-hidden"
                            };
                            rsx! {
                                a {
                                    key: "{team.id}",
                                    onclick: move |_| {
                                        team_dialog_signal.set(PopUpWindow::EditTeam(team_id));
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
                    team_dialog_signal,
                    on_teams_changed,
                }
            },
            PopUpWindow::EditTeam(team_id) => {
                match team_list.iter().find(|t| t.id == *team_id).cloned() {
                    Some(team_data) => rsx! {
                        EditTeamDialog {
                            team_dialog_signal,
                            project_id: cook_and_run_id,
                            team_data,
                            on_teams_changed,
                        }
                    },
                    // Team existiert nicht mehr (z. B. zwischenzeitlich
                    // gelöscht) – bewusst kein Signal-Write während des
                    // Renderns, einfach nichts anzeigen.
                    None => rsx! {},
                }
            }
            PopUpWindow::Share => {
                let share_config_option = match &*share_config.read() {
                    Some(Ok(ShareConfigState::Loaded(config))) => Some(config.clone()),
                    _ => None,
                };
                rsx! {
                    ShareDialog {
                        team_dialog_signal,
                        project_id: cook_and_run_id,
                        share_config_option,
                        number_of_teams,
                        on_share_config_changed,
                    }
                }
            }
        }
    }
}
