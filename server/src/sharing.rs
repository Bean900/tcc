use chrono::NaiveDateTime;
use tracing::event;
use uuid::Uuid;

use crate::{
    db::{self, Database},
    error::RestError,
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
}

pub fn get_by_id(db: &mut Database, config_id: &Uuid) -> Result<ShareTeamConfig, RestError> {
    let config = db.select_share(config_id).map_err(|e| {
        event!(
            tracing::Level::ERROR,
            "Database error while selecting share config for id {}: {}",
            config_id,
            e
        );
        RestError::InternalServer {
            message: "Database error while selecting share config".to_string(),
        }
    })?;

    Ok(ShareTeamConfig::from(config))
}
