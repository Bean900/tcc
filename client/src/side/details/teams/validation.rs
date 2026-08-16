//! Feldweise Validierung für das Team-Formular (Add + Edit teilen sich
//! diese Funktionen über `check_all`).

use crate::side::details::address::AddressParam;
use dioxus::prelude::*;

pub(super) fn check_all(
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

pub(super) fn check_team_name(
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

pub(super) fn check_team_email(
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

pub(super) fn check_team_tel(
    _team_tel_signal: Signal<String>,
    mut team_tel_error_signal: Signal<String>,
) -> bool {
    team_tel_error_signal.set(String::new());
    true
}

pub(super) fn check_members(members_signal: Signal<String>, mut members_error_signal: Signal<String>) -> bool {
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
