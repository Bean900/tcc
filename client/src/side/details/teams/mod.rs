//! Teams-Tab: Übersichts-Grid, Add-/Edit-Dialoge, Notizen und der
//! Share-Konfigurations-Dialog.
//!
//! Das Modul ist nach Verantwortlichkeit aufgeteilt – Details siehe die
//! einzelnen Dateien. `Teams` ist das einzige Symbol, das der Rest der
//! App kennen muss; alles andere ist internes Implementierungsdetail.

mod actions;
mod add_dialog;
mod card;
mod content;
mod edit_dialog;
mod form;
mod model;
mod notes;
mod share_dialog;
mod validation;

pub use model::TeamSortOption;

use crate::side::details::{ErrorPage, LoadingPage};
use crate::storage::{StorageManager, TeamData};
use content::TeamsContent;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Teams(cook_and_run_id: Uuid) -> Element {
    let storage = use_context::<Signal<StorageManager>>();

    let mut team_list: Resource<Result<(Vec<TeamData>, bool), String>> = use_resource(move || async move {
        let storage = storage.read().clone();
        let team_list = storage
            .select_cook_and_run_team_list(cook_and_run_id)
            .await?;
        let meta = storage.select_cook_and_run_meta(cook_and_run_id).await?;
        Ok((team_list, meta.is_in_cloud))
    });

    // Explizite Invalidierung statt impliziter Kopplung über das globale
    // Storage-Signal: Kind-Komponenten rufen diesen Callback gezielt nach
    // erfolgreichen Create/Update/Delete-Operationen auf.
    let on_teams_changed = move |_| {
        team_list.restart();
    };

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
                on_teams_changed,
            }
        ),
    }
}
