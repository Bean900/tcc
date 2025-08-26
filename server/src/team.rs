use chrono::NaiveDateTime;
use tracing::event;
use uuid::Uuid;

use crate::{
    address::{self, Address},
    db::{self, Database},
    error::RestError,
    note::{self, Note},
};

#[derive(Debug, Clone)]
pub struct Team {
    pub id: Uuid,
    pub created_by_user: Option<String>,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub address: Address,
    pub mail: Option<String>,
    pub phone: Option<String>,
    pub members: Option<u32>,
    pub diets: Option<String>,
    pub needs_check: bool,
    pub note_list: Vec<Note>,
}

impl Team {
    fn from(db_team: db::models::Team, address: Address, note_list: Vec<Note>) -> Self {
        Team {
            id: db_team.id,
            created_by_user: db_team.created_by_user,
            name: db_team.name,
            created: db_team.created,
            edited: db_team.edited,
            address,
            mail: db_team.mail,
            phone: db_team.phone,
            members: db_team.members.map(|m| m as u32),
            diets: db_team.diets,
            needs_check: db_team.needs_check,
            note_list,
        }
    }
}

pub fn get_list(db: &mut Database, cook_and_run_id: &Uuid) -> Result<Vec<Team>, RestError> {
    let team_vec = db.select_all_team(cook_and_run_id).map_err(|e| {
        event!(
            tracing::Level::ERROR,
            "Database error while selecting team list for cook and run id {}: {}",
            cook_and_run_id,
            e
        );
        RestError::InternalServer {
            message: "Database error while selecting team list!".to_string(),
        }
    })?;

    let mut result = Vec::new();
    for team in team_vec {
        let address = address::get_by_id(db, &team.address)?;
        let note_list = note::get_list_by_team_id(db, &team.id)?;
        result.push(Team::from(team, address, note_list));
    }
    Ok(result)
}
