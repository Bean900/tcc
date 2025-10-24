use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use tracing::{error, event, warn};
use uuid::Uuid;

use crate::{
    db::{self, models::Share, Database},
    error::{map_not_found_cook_and_run, RestError},
};

#[derive(Debug, Clone)]
pub struct ShareTeamConfig {
    pub id: Uuid,
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Vec<RequiredField>,
    pub max_teams: Option<u32>,
    pub registration_deadline: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum RequiredField {
    Mail,
    Phone,
    Members,
    Diets,
}
impl RequiredField {
    fn from(db_field: db::models::TeamFields) -> Self {
        match db_field {
            db::models::TeamFields::Mail => RequiredField::Mail,
            db::models::TeamFields::Phone => RequiredField::Phone,
            db::models::TeamFields::Members => RequiredField::Members,
            db::models::TeamFields::Diets => RequiredField::Diets,
        }
    }

    fn from_list(db_field_list: Option<Vec<Option<db::models::TeamFields>>>) -> Vec<Self> {
        db_field_list.map_or_else(
            || vec![],
            |list| {
                list.into_iter()
                    .filter_map(|f| f.map(RequiredField::from))
                    .collect()
            },
        )
    }
}

impl ShareTeamConfig {
    pub fn from(db_config: db::models::Share) -> Self {
        ShareTeamConfig {
            id: db_config.id,
            invite_text: db_config.invite_text,
            needs_login: db_config.needs_login,
            default_needs_check: db_config.default_needs_check,
            required_fields: RequiredField::from_list(db_config.required_fields),
            max_teams: db_config.max_teams.map(|m| m as u32),
            registration_deadline: db_config.registration_deadline,
            created: db_config.created,
        }
    }

    fn to_db(&self) -> db::models::Share {
        Share {
            id: self.id,
            created: self.created,
            invite_text: self.invite_text.clone(),
            needs_login: self.needs_login,
            default_needs_check: self.default_needs_check,
            required_fields: Some(
                self.required_fields
                    .iter()
                    .map(|f| match f {
                        RequiredField::Mail => db::models::TeamFields::Mail,
                        RequiredField::Phone => db::models::TeamFields::Phone,
                        RequiredField::Members => db::models::TeamFields::Members,
                        RequiredField::Diets => db::models::TeamFields::Diets,
                    })
                    .map(Some)
                    .collect(),
            ),
            max_teams: self.max_teams.map(|m| m as i32),
            registration_deadline: self.registration_deadline,
        }
    }
}

pub fn create(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    data: &ShareTeamConfig,
) -> Result<(), RestError> {
    match db.create_share(cook_and_run_id, user_id, &data.to_db()) {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            warn!("Could not create share in database due to unique violation");
            return Ok(());
        }
        Err(diesel::result::Error::NotFound) => {
            return Err(map_not_found_cook_and_run(
                cook_and_run_id,
                "create share",
                diesel::result::Error::NotFound,
            ));
        }
        Err(e) => {
            error!("Could not create share in database: {}", e);
            return Err(RestError::InternalServer {
                message: "Could not create share in database".to_string(),
            });
        }
    }
}

pub fn get_by_id(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<ShareTeamConfig, RestError> {
    let config = db
        .select_share(cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "get share", e)
            }
            _ => {
                error!(
                    "Could not get share of in cook and run project with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not get share of cook and run project with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })?;

    Ok(ShareTeamConfig::from(config))
}

pub(crate) fn delete(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    db.delete_share(cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "delete share", e)
            }
            _ => {
                error!(
                    "Could not delete share of in cook and run project with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete share of cook and run project with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })?;
    Ok(())
}
