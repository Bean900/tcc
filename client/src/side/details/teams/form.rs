//! Das eigentliche Formular mit den Team-Feldern (Name, Kontakt,
//! Mitgliederzahl, Diäten, Adresse). Wird sowohl vom Add- als auch vom
//! Edit-Dialog eingebettet – daher der Name "shared form".

use crate::side::details::address::{Address, AddressParam};
use crate::ui::forms::{Input, InputError, InputNumber, InputPhoneNumber};
use crate::ui::tokens::LBL;
use dioxus::prelude::*;
use uuid::Uuid;

use super::model::PopUpWindow;
use super::validation::{check_members, check_team_email, check_team_name, check_team_tel};

#[component]
pub(super) fn TeamDialog(
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
