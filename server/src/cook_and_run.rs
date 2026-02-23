use chrono::NaiveDateTime;
use diesel::result::DatabaseErrorKind;
use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    course::{self, Course},
    db::{self, models::CookAndRunUpdate, Database},
    error::{map_not_found_cook_and_run, RestError},
    plan::{self, Plan, PlanConfig},
    point::{self, Point},
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
    fn to_db(&self) -> CookAndRunUpdate {
        CookAndRunUpdate {
            name: &self.name,
            edited: &self.edited,
            occur: &self.occur,
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
    pub start_point: Option<Point>,
    pub end_point: Option<Point>,
    pub share_team_config: Option<ShareTeamConfig>,
    pub plan: Option<Plan>,
    pub plan_config: Option<PlanConfig>,
}

impl CookAndRun {
    fn from(
        cook_and_run: db::models::CookAndRun,
        team_list: Vec<Team>,
        course_list: Vec<Course>,
        start_point: Option<Point>,
        end_point: Option<Point>,
        share_team_config: Option<ShareTeamConfig>,
        plan: Option<Plan>,
        plan_config: Option<PlanConfig>,
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
            start_point,
            end_point,
            share_team_config,
            plan,
            plan_config,
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
    let team = team::get_list(db, cook_and_run_id, user_id)?;
    let course = course::get_list(db, cook_and_run_id, user_id)?;

    let start_point = cook_and_run
        .start_point
        .map(|a| point::get_by_id(db, &a))
        .transpose()?;

    let end_point = cook_and_run
        .end_point
        .map(|a| point::get_by_id(db, &a))
        .transpose()?;

    let share_team_config = cook_and_run
        .share_team_config
        .map(|_| sharing::get_by_id(db, cook_and_run_id, user_id))
        .transpose()?;

    Ok(CookAndRun::from(
        cook_and_run,
        team,
        course,
        start_point,
        end_point,
        share_team_config,
        None,
        None,
    ))
}

pub fn get_cook_and_run_meta(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<CookAndRunMeta, RestError> {
    db.select_cook_and_run(cook_and_run_id, user_id)
        .map(CookAndRunMeta::from)
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
        })
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

pub fn update_cook_and_run_meta(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    meta: &CookAndRunMeta,
) -> Result<(), RestError> {
    db.update_cook_and_run_meta(cook_and_run_id, user_id, &meta.to_db())
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

pub fn get_cook_and_run_start_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<Option<Point>, RestError> {
    db.select_cook_and_run_start_point_id(cook_and_run_id, user_id)
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
        })?
        .map(|point_id| point::get_by_id(db, &point_id))
        .transpose()
}

pub fn set_cook_and_run_start_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    point: &Point,
) -> Result<(), RestError> {
    db.set_cook_and_run_start_point(
        cook_and_run_id,
        user_id,
        &point.to_db(),
        &point.address.to_db(),
    )
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

pub fn get_cook_and_run_end_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<Option<Point>, RestError> {
    db.select_cook_and_run_end_point_id(cook_and_run_id, user_id)
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
        })?
        .map(|point_id| point::get_by_id(db, &point_id))
        .transpose()
}

pub fn set_cook_and_run_end_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    point: &Point,
) -> Result<(), RestError> {
    db.set_cook_and_run_end_point(
        cook_and_run_id,
        user_id,
        &point.to_db(),
        &point.address.to_db(),
    )
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
    db.delete_cook_and_run_start_point(cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "delete cook and run start point", e)
            }
            _ => {
                error!(
                    "Could not delete cook and run project start point with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete cook and run project start point with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })
}

pub fn delete_cook_and_run_end_point(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    db.delete_cook_and_run_end_point(cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "delete cook and run end point", e)
            }
            _ => {
                error!(
                    "Could not delete cook and run project end point with id {} in database: {}",
                    cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete cook and run project end point with id {} in database",
                        cook_and_run_id
                    ),
                }
            }
        })
}
