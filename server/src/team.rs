use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    address::Address,
    cook_and_run::get_cook_and_run,
    db::{self, Database},
    error::{map_not_found_cook_and_run, RestError},
    note::{get_list_by_team_id, Note},
};

#[derive(Debug, Clone)]
pub struct Team {
    pub id: Uuid,
    pub cook_and_run_id: Uuid,
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
    fn from(db_team: db::models::Team, address: db::models::Address, note_list: Vec<Note>) -> Self {
        Team {
            id: db_team.id,
            cook_and_run_id: db_team.cook_and_run_id,
            created_by_user: db_team.created_by_user,
            name: db_team.name,
            created: db_team.created,
            edited: db_team.edited,
            address: Address::from(address),
            mail: db_team.mail,
            phone: db_team.phone,
            members: db_team.members.map(|m| m as u32),
            diets: db_team.diets,
            needs_check: db_team.needs_check,
            note_list,
        }
    }

    fn to(&self) -> db::models::Team {
        db::models::Team {
            id: self.id,
            cook_and_run_id: self.cook_and_run_id,
            created_by_user: self.created_by_user.clone(),
            name: self.name.clone(),
            created: self.created,
            edited: self.edited,
            mail: self.mail.clone(),
            phone: self.phone.clone(),
            members: self.members.map(|m| m as i32),
            diets: self.diets.clone(),
            needs_check: self.needs_check,
            address: self.address.id,
        }
    }
}

pub(crate) fn get_list(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<Vec<Team>, RestError> {
    let team_list = db
        .select_all_team(cook_and_run_id, user_id)
        .map_err(|e| {
            error!(
                "Database error while selecting team list for cook and run id {}: {}",
                cook_and_run_id, e
            );
            RestError::InternalServer {
                message: "Database error while selecting team list!".to_string(),
            }
        })?
        .into_iter()
        .map(|team_address| {
            let note_list = get_list_by_team_id(db, &team_address.0.id)?;
            Ok(Team::from(team_address.0, team_address.1, note_list))
        })
        .collect::<Result<Vec<_>, RestError>>()?;
    Ok(team_list)
}

pub(crate) fn get(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    team_id: &Uuid,
) -> Result<Team, RestError> {
    let (team, address) =
        db.select_team(team_id, cook_and_run_id, user_id)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    map_not_found_cook_and_run(cook_and_run_id, "loading team", e)
                }
                _ => {
                    error!(
                        "Could not get team {} in project with id {} from database: {}",
                        team_id, cook_and_run_id, e
                    );
                    RestError::InternalServer {
                        message: format!(
                        "Could not get team {} in cook and run project with id {} from database",
                        team_id, cook_and_run_id
                    ),
                    }
                }
            })?;
    Ok(Team::from(team, address, get_list_by_team_id(db, team_id)?))
}

pub(crate) fn delete(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    team_id: &Uuid,
) -> Result<(), RestError> {
    db.delete_team(team_id, cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "delete team", e)
            }
            _ => {
                error!(
                    "Could not delete team {} in cook and run project with id {} in database: {}",
                    team_id, cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete team {} cook and run project with id {} in database",
                        team_id, cook_and_run_id
                    ),
                }
            }
        })?;
    Ok(())
}

pub(crate) fn update(db: &mut Database, user_id: &str, data: &Team) -> Result<(), RestError> {
    db.update_team(&data.to(), &data.address.to_db(), user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(&data.cook_and_run_id, "updating team", e)
            }
            _ => {
                error!(
                    "Could not update team {} in cook and run project with id {} in database: {}",
                    data.id, data.cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not update team {} in cook and run project with id {} in database",
                        data.id, data.cook_and_run_id
                    ),
                }
            }
        })
}

pub fn create(db: &mut Database, user_id: &str, data: &Team) -> Result<(), RestError> {
    let _ = get_cook_and_run(db, &data.cook_and_run_id, user_id)?;

    match db.create_team(&data.to(), &data.address.to_db()) {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            warn!("Could not create team in database due to unique violation");
            return Ok(());
        }
        Err(e) => {
            error!("Could not create team in database: {}", e);
            return Err(RestError::InternalServer {
                message: "Could not create team in database".to_string(),
            });
        }
    }
}
