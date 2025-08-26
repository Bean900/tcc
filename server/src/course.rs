use tracing::event;
use uuid::Uuid;

use crate::{
    db::{self, Database},
    error::RestError,
};
#[derive(Debug, Clone)]
pub struct Course {
    pub id: Uuid,
    pub name: String,
    pub time: String,
}

impl Course {
    pub fn from(db_course: db::models::Course) -> Self {
        Course {
            id: db_course.id,
            name: db_course.name,
            time: db_course.time,
        }
    }
}

pub(crate) fn get_list(
    db: &mut Database,
    cook_and_run_id: &Uuid,
) -> Result<Vec<Course>, RestError> {
    let course_list = db
        .select_all_course(cook_and_run_id)
        .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while selecting course list for cook and run id {}: {}",
                cook_and_run_id,
                e
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
