use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::{
    address::Address,
    cook_and_run::get_cook_and_run,
    db::{self, Database},
    note::{get_list_by_team_id, Note},
    rest_error::{map_not_found_cook_and_run, RestError},
    sharing::ShareTeamConfig,
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
                operation = "Find Project",
                "Database error while selecting team list for cook and run id {}: {}",
                cook_and_run_id,
                e
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
                        operation = "Find Team",
                        "Could not get team {} in project with id {} from database: {}",
                        team_id,
                        cook_and_run_id,
                        e
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
                    operation = "Delete Team",
                    "Could not delete team {} in cook and run project with id {} in database: {}",
                    team_id,
                    cook_and_run_id,
                    e
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
                    operation = "Update Team",
                    "Could not update team {} in cook and run project with id {} in database: {}",
                    data.id,
                    data.cook_and_run_id,
                    e
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

pub fn create(db: &mut Database, user_id: &Option<String>, data: &Team) -> Result<(), RestError> {
    match db.select_share_uncheckt(&data.cook_and_run_id) {
        Ok(share) => check_team_against_share(db, &ShareTeamConfig::from(share), user_id, data)?,
        Err(diesel::result::Error::NotFound) => {
            if let Some(user_id) = user_id {
                debug!(
                    operation = "Create Team with user_id - Find share",
                    "No share config found for cook and run id {}, checking if user is owner",
                    data.cook_and_run_id
                );
                let _ = get_cook_and_run(db, &data.cook_and_run_id, user_id)?;
            } else {
                warn!(
                    operation = "Create Team without user_id",
                    "Could not find share config for cook and run id {}, and no user id provided",
                    data.cook_and_run_id
                );
                return Err(RestError::NotFound {
                    message: "Could not find cook and run project or share".to_string(),
                });
            }
        }
        Err(e) => {
            error!(
                operation = "Create Team - Find share config",
                "Database error while selecting share config for id {}: {}",
                data.cook_and_run_id,
                e
            );
            return Err(RestError::InternalServer {
                message: "Database error while selecting share config".to_string(),
            });
        }
    }

    match db.create_team(&data.to(), &data.address.to_db()) {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            warn!(
                operation = "Create Team",
                "Could not create team in database due to unique violation"
            );
            return Ok(());
        }
        Err(e) => {
            error!(
                operation = "Create Team",
                "Could not create team in database: {}", e
            );
            return Err(RestError::InternalServer {
                message: "Could not create team in database".to_string(),
            });
        }
    }
}

fn check_team_against_share(
    db: &mut Database,
    share: &ShareTeamConfig,
    user_id: &Option<String>,
    data: &Team,
) -> Result<(), RestError> {
    debug!("Checking team against share config: {:?}", share);
    if user_id
        .clone()
        .is_some_and(|user_id| get_cook_and_run(db, &data.cook_and_run_id, &user_id).is_ok())
    {
        debug!("User is the owner of the cook and run project, skipping share checks");
        return Ok(());
    }

    debug!("User is not the owner of the cook and run project, performing share checks");

    let deadline = share.registration_deadline;
    if deadline.is_some_and(|deadline| deadline < chrono::Utc::now().naive_utc()) {
        warn!(
            operation = "Check deadline",
            "Registration deadline has passed: {:?}", deadline
        );
        return Err(RestError::Forbidden {
            message: "The registration deadline has passed".to_string(),
        });
    }

    if share.needs_login && user_id.is_none() {
        warn!(
            operation = "Check login status",
            "User is not logged in, but login is required to register a team"
        );
        return Err(RestError::Unprocessable {
            message: "You need to be logged in to register a team".to_string(),
        });
    }

    if let Some(max_team_size) = share.max_teams {
        let team_size = db.count_teams(&data.cook_and_run_id).map_err(|e| {
            error!(
                operation = "Check max teams",
                "Could not get team count from database: {}", e
            );
            RestError::InternalServer {
                message: "Could not create team in database".to_string(),
            }
        })?;
        if team_size >= max_team_size as i64 {
            warn!(
                operation = "Check max teams",
                "Maximum number of teams reached: {}", max_team_size
            );
            return Err(RestError::Forbidden {
                message: "The maximum number of teams has been reached".to_string(),
            });
        }
    }

    if share.default_needs_check && !data.needs_check {
        warn!(
            operation = "Check needs_check",
            "The needs_check field must be true, but is false"
        );
        return Err(RestError::Unprocessable {
            message: "The needs_check field must be true".to_string(),
        });
    }

    for required_field in share.required_fields.iter() {
        match required_field {
            crate::sharing::RequiredField::Mail => {
                if data.mail.is_none() {
                    warn!(operation = "Check mail", "The mail field is required");
                    return Err(RestError::Unprocessable {
                        message: "The mail field is required".to_string(),
                    });
                }
            }
            crate::sharing::RequiredField::Phone => {
                if data.phone.is_none() {
                    warn!(operation = "Check phone", "The phone field is required");
                    return Err(RestError::Unprocessable {
                        message: "The phone field is required".to_string(),
                    });
                }
            }
            crate::sharing::RequiredField::Members => {
                if data.members.is_none() {
                    warn!(operation = "Check members", "The members field is required");
                    return Err(RestError::Unprocessable {
                        message: "The members field is required".to_string(),
                    });
                }
            }
            crate::sharing::RequiredField::Diets => {
                if data.diets.is_none() {
                    warn!(operation = "Check diets", "The diets field is required");
                    return Err(RestError::Unprocessable {
                        message: "The diets field is required".to_string(),
                    });
                }
            }
        }
    }
    Ok(())
}
