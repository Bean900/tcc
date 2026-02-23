use std::collections::HashMap;

use chrono::NaiveDate;
use tracing::event;
use uuid::Uuid;

use crate::{
    db::{self, Database},
    error::RestError,
};

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

    fn from_list(db_field_list: Vec<Option<db::models::Access>>) -> Vec<Self> {
        db_field_list
            .into_iter()
            .filter_map(|f| f.map(Access::from))
            .collect()
    }

    fn to_db(&self) -> db::models::Access {
        match self {
            Access::Link => db::models::Access::Link,
            Access::Account => db::models::Access::Account,
        }
    }

    fn to_db_list(access_list: &[Access]) -> Vec<Option<db::models::Access>> {
        access_list.iter().map(|a| Some(a.to_db())).collect()
    }
}

#[derive(Debug, Clone)]
pub enum Language {
    DEUTSCH,
    ENGLISH,
}

impl Language {
    fn from(db_field: db::models::Language) -> Self {
        match db_field {
            db::models::Language::DEUTSCH => Language::DEUTSCH,
            db::models::Language::ENGLISH => Language::ENGLISH,
        }
    }

    fn to_db(&self) -> db::models::Language {
        match self {
            Language::DEUTSCH => db::models::Language::DEUTSCH,
            Language::ENGLISH => db::models::Language::ENGLISH,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlanConfig {
    access: Vec<Access>,
    title: String,
    description: String,
    date: NaiveDate,
    language: Language,
}

impl PlanConfig {
    pub fn from(db_plan_config: db::models::PlanConfig) -> Self {
        PlanConfig {
            access: Access::from_list(db_plan_config.access),
            title: db_plan_config.title,
            description: db_plan_config.description,
            date: db_plan_config.date,
            language: Language::from(db_plan_config.language),
        }
    }

    pub fn to_db(&self, id: Uuid) -> db::models::PlanConfig {
        db::models::PlanConfig {
            id,
            access: Access::to_db_list(&self.access),
            title: self.title.clone(),
            description: self.description.clone(),
            date: self.date,
            language: self.language.to_db(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hosting {
    pub id: Uuid,
    pub name: Uuid,            // Course ID
    pub host: Uuid,            // Team ID
    pub guest_list: Vec<Uuid>, // Team ID
}

impl Hosting {
    pub fn from(db_hosting: db::models::HostingData) -> Self {
        Hosting {
            id: db_hosting.id,
            name: db_hosting.name,
            host: db_hosting.host,
            guest_list: db_hosting.guest_list,
        }
    }

    pub fn to_db(&self) -> db::models::HostingData {
        db::models::HostingData {
            id: self.id,
            name: self.name,
            host: self.host,
            guest_list: self.guest_list.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub id: Uuid,
    pub hosting_list: Vec<Hosting>,
    pub walking_path: HashMap<Uuid, Vec<Uuid>>,
}

impl Plan {
    pub fn from(db_plan: db::models::Plan) -> Self {
        Plan {
            id: db_plan.id,
            hosting_list: db_plan
                .data
                .hosting_list
                .into_iter()
                .map(Hosting::from)
                .collect(),
            walking_path: db_plan.data.walking_path,
        }
    }

    pub fn to_db(&self, id: Uuid) -> db::models::Plan {
        db::models::Plan {
            id,
            data: db::models::PlanData {
                hosting_list: self.hosting_list.iter().map(Hosting::to_db).collect(),
                walking_path: self.walking_path.clone(),
            },
        }
    }
}

pub fn get_by_id(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<Plan, RestError> {
    let db_plan = db.select_plan(cook_and_run_id, user_id).map_err(|e| {
        event!(
            tracing::Level::ERROR,
            "Database error while selecting plan for cook_and_run_id: {}, user_id: {}: {}",
            cook_and_run_id,
            user_id,
            e
        );
        RestError::InternalServer {
            message: "Database error while selecting plan!".to_string(),
        }
    })?;
    Ok(Plan::from(db_plan))
}

pub fn get_config_by_id(
    db: &mut Database,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<PlanConfig, RestError> {
    let db_plan_config = db.select_plan_config(cook_and_run_id, user_id)  .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while selecting plan config for cook_and_run_id: {}, user_id: {}: {}",
                cook_and_run_id,
                user_id,
                e
            );
            RestError::InternalServer {
                message: "Database error while selecting plan config!".to_string(),
            }
        })?;
    Ok(PlanConfig::from(db_plan_config))
}

pub fn create_or_update(
    db: &mut Database,
    plan: Plan,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    let plan_id = Uuid::new_v4();
    db.create_plan(plan.to_db(plan_id), cook_and_run_id, user_id)
        .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while creating/updating plan for cook_and_run_id: {}, user_id: {}: {}",
                cook_and_run_id,
                user_id,
                e
            );
            RestError::InternalServer {
                message: "Database error while creating/updating plan!".to_string(),
            }
        })
}

pub fn create_or_update_config(
    db: &mut Database,
    plan_config: PlanConfig,
    cook_and_run_id: &Uuid,
    user_id: &str,
) -> Result<(), RestError> {
    let plan_config_id = Uuid::new_v4();
    db.create_plan_config(plan_config.to_db(plan_config_id), cook_and_run_id, user_id)
        .map_err(|e| {
            event!(
                tracing::Level::ERROR,
                "Database error while creating/updating plan config for cook_and_run_id: {}, user_id: {}: {}",
                cook_and_run_id,
                user_id,
                e
            );
            RestError::InternalServer {
                message: "Database error while creating/updating plan config!".to_string(),
            }
        })
}
