use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    address::{self, Address},
    course::{self, Course},
    db::{self, Database},
    error::{map_not_found_cook_and_run, RestError},
    plan::{self, Plan},
    sharing::{self, ShareTeamConfig},
    team::{self, Team},
};

// Cook and Run models
#[derive(Debug, Clone)]
pub struct CookAndRunMeta {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
}

impl CookAndRunMeta {
    fn from(cook_and_run: db::models::CookAndRun) -> Self {
        CookAndRunMeta {
            id: cook_and_run.id,
            user_id: cook_and_run.user_id,
            name: cook_and_run.name,
            created: cook_and_run.created,
            edited: cook_and_run.edited,
            occur: cook_and_run.occur,
        }
    }
}

pub struct CookAndRunCreate<'a> {
    pub id: &'a Uuid,
    pub user_id: &'a str,
    pub name: &'a str,
    pub created: &'a NaiveDateTime,
    pub edited: &'a NaiveDateTime,
    pub occur: &'a NaiveDateTime,
}

impl<'a> CookAndRunCreate<'a> {
    fn to(&self) -> db::models::CookAndRunCreate {
        db::models::CookAndRunCreate {
            id: &self.id,
            user_id: &self.user_id,
            name: &self.name,
            created: &self.created,
            edited: &self.edited,
            occur: &self.occur,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CookAndRun {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub created: NaiveDateTime,
    pub edited: NaiveDateTime,
    pub occur: NaiveDateTime,
    pub team_list: Vec<Team>,
    pub course_list: Vec<Course>,
    pub course_with_multiple_hosts: Option<Uuid>,
    pub start_point: Option<Address>,
    pub end_point: Option<Address>,
    pub share_team_config: Option<ShareTeamConfig>,
    pub plan: Option<Plan>,
}

impl CookAndRun {
    fn from(
        cook_and_run: db::models::CookAndRun,
        team_list: Vec<Team>,
        course_list: Vec<Course>,
        start_point: Option<Address>,
        end_point: Option<Address>,
        share_team_config: Option<ShareTeamConfig>,
        plan: Option<Plan>,
    ) -> Self {
        CookAndRun {
            id: cook_and_run.id,
            user_id: cook_and_run.user_id,
            name: cook_and_run.name,
            created: cook_and_run.created,
            edited: cook_and_run.edited,
            occur: cook_and_run.occur,
            team_list,
            course_list,
            course_with_multiple_hosts: cook_and_run.course_with_multiple_hosts,
            start_point,
            end_point,
            share_team_config,
            plan,
        }
    }
}

pub fn get_list_of_cook_and_run_meta(
    db: &mut Database,
    user_id: &str,
) -> Result<Vec<CookAndRunMeta>, RestError> {
    db.select_all_cook_and_run(user_id)
        .map(|list| list.into_iter().map(CookAndRunMeta::from).collect())
        .map_err(|e| {
            error!(
                "Could not get list of cook and run projects from database: {}",
                e
            );
            RestError::InternalServer {
                message: "Could not get list of cook and run projects from database".to_string(),
            }
        })
}

pub fn get_cook_and_run(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<CookAndRun, RestError> {
    let cook_and_run = db
        .select_cook_and_run(cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "loading cook and run", e)
            }
            _ => {
                error!(
                    "Could not get cook and run project with id {} from database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not get cook and run project with id {} from database",
                        cook_and_run_id
                    ),
                }
            }
        })?;
    let team = team::get_list(db, cook_and_run_id)?;
    let course = course::get_list(db, cook_and_run_id)?;

    let start_point = cook_and_run
        .start_point
        .map(|a| address::get_by_id(db, &a))
        .transpose()?;

    let end_point = cook_and_run
        .end_point
        .map(|a| address::get_by_id(db, &a))
        .transpose()?;

    let share_team_config = cook_and_run
        .share_team_config
        .map(|s| sharing::get_by_id(db, &s))
        .transpose()?;

    let plan = cook_and_run
        .plan
        .map(|p| plan::get_by_id(db, &p))
        .transpose()?;

    Ok(CookAndRun::from(
        cook_and_run,
        team,
        course,
        start_point,
        end_point,
        share_team_config,
        plan,
    ))
}

pub fn create_cook_and_run(
    db: &mut Database,
    cook_and_run: CookAndRunCreate,
) -> Result<(), RestError> {
    match db.create_cook_and_run(&cook_and_run.to()) {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            warn!("Could not create cook and run project in database due to unique violation");
            return Ok(());
        }
        Err(e) => {
            error!("Could not create cook and run project in database: {}", e);
            return Err(RestError::InternalServer {
                message: "Could not create cook and run project in database".to_string(),
            });
        }
    }
}

pub fn delete_cook_and_run(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    db.delete_cook_and_run(cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "deleting cook and run", e)
            }
            _ => {
                error!(
                    "Could not delete cook and run project with id {} from database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete cook and run project with id {} from database",
                        cook_and_run_id
                    ),
                }
            }
        })
}

pub fn update_cook_and_run_name(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    new_name: &str,
) -> Result<(), RestError> {
    db.update_cook_and_run_name(cook_and_run_id, user_id, new_name)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "updating cook and run", e)
            }
            _ => {
                error!(
                    "Could not update cook and run project with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not update cook and run project with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })
}

pub fn set_cook_and_run_start_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    address: &Address,
) -> Result<(), RestError> {
    db.set_cook_and_run_start_point(cook_and_run_id, user_id, &address.to_db())
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "set cook and run start point", e)
            }
            _ => {
                error!(
                    "Could not update cook and run project start point with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not update cook and run project start point with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })
}

pub fn set_cook_and_run_end_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    address: &Address,
) -> Result<(), RestError> {
    db.set_cook_and_run_end_point(cook_and_run_id, user_id, &address.to_db())
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "set cook and run end point", e)
            }
            _ => {
                error!(
                    "Could not update cook and run project end point with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not update cook and run project end point with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })
}

pub fn delete_cook_and_run_start_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    let cook_and_run = get_cook_and_run(db, cook_and_run_id, user_id)?;
    let addr = cook_and_run.start_point.ok_or_else(|| {
        warn!(
            "Could not find start point in cook and run project with id {}!",
            cook_and_run_id
        );
        RestError::NotFound {
            message: format!(
                "Start point of cook and run project with id {} not found in database",
                cook_and_run_id
            ),
        }
    })?;

    db.delete_address(&addr.id).map_err(|e| {
        error!(
            "Could not delete start point in cook and run project with id {}: {}",
            cook_and_run_id, e
        );
        RestError::InternalServer {
            message: format!(
                "Could not delete start point in cook and run project with id {}",
                cook_and_run_id
            ),
        }
    })
}

pub fn delete_cook_and_run_end_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    let cook_and_run = get_cook_and_run(db, cook_and_run_id, user_id)?;
    let addr = cook_and_run.end_point.ok_or_else(|| {
        warn!(
            "Could not find end point in cook and run project with id {}!",
            cook_and_run_id
        );
        RestError::NotFound {
            message: format!(
                "End point of cook and run project with id {} not found in database",
                cook_and_run_id
            ),
        }
    })?;

    db.delete_address(&addr.id).map_err(|e| {
        error!(
            "Could not delete end point in cook and run project with id {}: {}",
            cook_and_run_id, e
        );
        RestError::InternalServer {
            message: format!(
                "Could not delete end point in cook and run project with id {}",
                cook_and_run_id
            ),
        }
    })
}
