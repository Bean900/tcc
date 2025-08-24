use uuid::Uuid;

use crate::db::{self, Database};
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

pub(crate) fn get_list(db: &mut Database, cook_and_run_id: &Uuid) -> Result<Vec<Course>, String> {
    let course_list = db
        .select_all_course(cook_and_run_id)?
        .into_iter()
        .map(Course::from)
        .collect();
    Ok(course_list)
}
