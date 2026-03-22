use diesel::result::DatabaseErrorKind;
use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    cook_and_run::get_cook_and_run,
    db::{self, Database},
    rest_error::{map_not_found_cook_and_run, RestError},
};
#[derive(Debug, Clone)]
pub struct Course {
    pub id: Uuid,
    pub cook_and_run_id: Uuid,
    pub name: String,
    pub time: String,
    pub has_multiple_hosts: bool,
}

impl Course {
    fn from(db_course: db::models::Course) -> Self {
        Course {
            id: db_course.id,
            cook_and_run_id: db_course.cook_and_run_id,
            name: db_course.name,
            time: db_course.time,
            has_multiple_hosts: db_course.has_multiple_hosts,
        }
    }

    fn to(&self) -> db::models::Course {
        db::models::Course {
            id: self.id,
            cook_and_run_id: self.cook_and_run_id,
            name: self.name.clone(),
            time: self.time.clone(),
            has_multiple_hosts: self.has_multiple_hosts,
        }
    }
}

pub(crate) fn get_list(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<Vec<Course>, RestError> {
    let course_list = db
        .select_all_course(cook_and_run_id, user_id)
        .map_err(|e| {
            error!(
                "Database error while selecting course list for cook and run id {}: {}",
                cook_and_run_id, e
            );
            RestError::InternalServer {
                message: "Database error while selecting course list!".to_string(),
            }
        })?
        .into_iter()
        .map(Course::from)
        .collect();
    Ok(course_list)
}

pub(crate) fn get(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    course_id: &Uuid,
) -> Result<Course, RestError> {
    let course = db
        .select_course(course_id, cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "loading course", e)
            }
            _ => {
                error!(
                    "Could not get course {} in project with id {} from database: {}",
                    course_id, cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not get course {} in cook and run project with id {} from database",
                        course_id, cook_and_run_id
                    ),
                }
            }
        })?;
    Ok(Course::from(course))
}

pub(crate) fn delete(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
    course_id: &Uuid,
) -> Result<(), RestError> {
    db.delete_course(course_id, cook_and_run_id, user_id)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                map_not_found_cook_and_run(cook_and_run_id, "delete course", e)
            }
            _ => {
                error!(
                    "Could not delete course {} in cook and run project with id {} in database: {}",
                    course_id, cook_and_run_id, e
                );
                RestError::InternalServer {
                    message: format!(
                        "Could not delete course {} cook and run project with id {} in database",
                        course_id, cook_and_run_id
                    ),
                }
            }
        })?;
    Ok(())
}

pub(crate) fn update(db: &mut Database, user_id: &str, data: &Course) -> Result<(), RestError> {
    db.update_course(&data.to(), user_id).map_err(|e| match e {
        diesel::result::Error::NotFound => {
            map_not_found_cook_and_run(&data.cook_and_run_id, "updating course", e)
        }
        _ => {
            error!(
                "Could not update course {} in cook and run project with id {} in database: {}",
                data.id, data.cook_and_run_id, e
            );
            RestError::InternalServer {
                message: format!(
                    "Could not update course {} in cook and run project with id {} in database",
                    data.id, data.cook_and_run_id
                ),
            }
        }
    })
}

pub fn create(db: &mut Database, user_id: &str, data: &Course) -> Result<(), RestError> {
    let _ = get_cook_and_run(db, &data.cook_and_run_id, user_id)?;
    match db.create_course(&data.to()) {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            warn!(
                operation = "Create Course",
                "Could not create course in database due to unique violation"
            );
            return Ok(());
        }
        Err(e) => {
            error!(
                operation = "Create Course",
                "Could not create course in database: {}", e
            );
            return Err(RestError::InternalServer {
                message: "Could not create course in database".to_string(),
            });
        }
    }
}
