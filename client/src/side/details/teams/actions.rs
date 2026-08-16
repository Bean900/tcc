//! Storage-Mutationen für Teams und ihre Notizen.
//!
//! `storage_signal` wird bewusst als Parameter übergeben statt per
//! `use_context` innerhalb dieser freien (nicht `use_`-präfixierten)
//! Funktionen abgefragt – das vermeidet einen Verstoß gegen die Rules
//! of Hooks.

use crate::storage::{AddressData, NoteCreate, StorageManager, TeamCreate, TeamUpdate};
use dioxus::prelude::*;
use uuid::Uuid;

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

pub(super) async fn add_team(
    mut storage_signal: Signal<StorageManager>,
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
    let mut storage = storage_signal.write();
    storage
        .create_team_of_cook_and_run(id, Uuid::new_v4(), &team)
        .await
}

pub(super) async fn update_team(
    mut storage_signal: Signal<StorageManager>,
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
    let mut storage = storage_signal.write();
    storage
        .update_team_of_cook_and_run(id, team_id, &team)
        .await
}

pub(super) async fn add_team_note(
    mut storage_signal: Signal<StorageManager>,
    id: Uuid,
    team_id: Uuid,
    headline: String,
    content: String,
) -> Result<(), String> {
    let note = NoteCreate { headline, content };
    let mut storage = storage_signal.write();
    storage
        .create_team_note_of_cook_and_run(id, team_id, Uuid::new_v4(), &note)
        .await
}

pub(super) async fn delete_team(
    mut storage_signal: Signal<StorageManager>,
    id: Uuid,
    team_id: Uuid,
) -> Result<(), String> {
    let mut storage = storage_signal.write();
    storage.delete_team_of_cook_and_run(id, team_id).await
}
