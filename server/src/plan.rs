use std::collections::HashMap;

use tracing::event;
use uuid::Uuid;

use crate::{db, error::RestError};

#[derive(Debug, Clone)]
pub enum Access {
    Link,
    Account,
}

impl Access {
    fn from(db_field: db::models::Access) -> Self {
        match db_field {
            db::models::Access::Link => Access::Link,
            db::models::Access::Account => Access::Account,
        }
    }

    fn from_list(db_field_list: Option<Vec<Option<db::models::Access>>>) -> Vec<Self> {
        db_field_list.map_or_else(
            || vec![],
            |list| {
                list.into_iter()
                    .filter_map(|f| f.map(Access::from))
                    .collect()
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub access: Vec<Access>,
    pub introduction: Option<String>,
    pub hosting_assignments: Vec<Hosting>,
    pub walking_paths: HashMap<Uuid, Vec<WalkingPathStep>>,
}

impl Plan {
    pub fn from(
        db_plan: crate::db::models::Plan,
        hosting_list: Vec<Hosting>,
        walking_paths: HashMap<Uuid, Vec<WalkingPathStep>>,
    ) -> Self {
        Plan {
            access: Access::from_list(db_plan.access),
            introduction: db_plan.introduction,
            hosting_assignments: hosting_list,
            walking_paths,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hosting {
    pub id: Uuid,
    pub course_id: Uuid,
    pub team_id: Uuid,
    pub guest_team_ids: Vec<Uuid>,
}

impl Hosting {
    fn from(db_hosting: crate::db::models::Hosting) -> Self {
        let guest_team_ids: Vec<Uuid> = serde_json::from_value(db_hosting.guest_team_ids)
            .expect("Failed to deserialize guest_team_ids");
        Hosting {
            id: db_hosting.id,
            course_id: db_hosting.course_id,
            team_id: db_hosting.team_id,
            guest_team_ids,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WalkingPathStep {
    pub course_id: Uuid,
    pub host_team_id: Uuid,
}

impl WalkingPathStep {
    fn from_map(db_step: serde_json::Value) -> HashMap<Uuid, Vec<Self>> {
        let walking_path: HashMap<Uuid, Vec<crate::db::models::WalkingPathStep>> =
            serde_json::from_value(db_step).expect("Failed to deserialize WalkingPathStep");

        walking_path
            .into_iter()
            .map(|(key, steps)| {
                let step_list = steps.into_iter().map(WalkingPathStep::from).collect();
                (key, step_list)
            })
            .collect()
    }

    fn from(db_step: crate::db::models::WalkingPathStep) -> Self {
        WalkingPathStep {
            course_id: db_step.course_id,
            host_team_id: db_step.host_team_id,
        }
    }
}

pub fn get_by_id(db: &mut crate::db::Database, plan_id: &Uuid) -> Result<Plan, RestError> {
    let db_plan = db.select_plan(plan_id).map_err(|e| {
        event!(
            tracing::Level::ERROR,
            "Database error while selecting plan for id {}: {}",
            plan_id,
            e
        );
        RestError::InternalServer {
            message: "Database error while selecting plan!".to_string(),
        }
    })?;

    let hosting_assignments = db
        .select_all_hosting(plan_id)
        .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while selecting hosting assignments for plan id {}: {}",
                plan_id,
                e
            );
            RestError::InternalServer {
                message: "Database error while selecting hosting assignments!".to_string(),
            }
        })?
        .into_iter()
        .map(Hosting::from)
        .collect();

    let walking_paths = WalkingPathStep::from_map(db_plan.walking_paths);

    let access = Access::from_list(db_plan.access);

    Ok(Plan {
        access,
        introduction: db_plan.introduction,
        hosting_assignments,
        walking_paths,
    })
}
