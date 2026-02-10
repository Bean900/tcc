use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::{IntoResponse, Json, Response},
    routing::{delete, get, patch, post},
    Extension, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::RestError,
    rest::{
        auth::{require_permission, Claims, READ_PERMISSION, UPDATE_PERMISSION},
        models::{RequiredField, ShareTeamConfig},
    },
    sharing::{self},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateShareConfigRequest {
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Vec<RequiredField>,
    pub max_teams: Option<u32>,
    pub registration_deadline: Option<NaiveDateTime>,
}

impl CreateShareConfigRequest {
    pub fn to(
        &self,
        share_id: &Uuid,
        time: &chrono::NaiveDateTime,
    ) -> crate::sharing::ShareTeamConfig {
        crate::sharing::ShareTeamConfig {
            id: *share_id,
            invite_text: self.invite_text.clone(),
            needs_login: self.needs_login,
            default_needs_check: self.default_needs_check,
            required_fields: self
                .required_fields
                .iter()
                .map(|f| match f {
                    RequiredField::Mail => crate::sharing::RequiredField::Mail,
                    RequiredField::Phone => crate::sharing::RequiredField::Phone,
                    RequiredField::Members => crate::sharing::RequiredField::Members,
                    RequiredField::Diets => crate::sharing::RequiredField::Diets,
                })
                .collect(),
            max_teams: self.max_teams,
            registration_deadline: self.registration_deadline.clone(),
            created: *time,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ShareConfigResponse {
    #[serde(flatten)]
    pub config: ShareTeamConfig,
    pub share_url: String,
    pub registration_count: Option<u32>,
}

impl IntoResponse for ShareConfigResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            post(create_share_config).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            patch(update_share_config).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            get(get_share_config).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            delete(delete_share_config).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
}

/// Create team sharing configuration
async fn create_share_config(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<CreateShareConfigRequest>,
) -> Result<(), RestError> {
    let time = chrono::Utc::now().naive_utc();

    sharing::create(
        &mut state.db,
        &cook_and_run_id,
        &claims.sub,
        &payload.to(&Uuid::new_v4(), &time),
    )?;

    Ok(())
}

/// Update team sharing configuration
async fn update_share_config(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<CreateShareConfigRequest>,
) -> Result<(), RestError> {
    let time = chrono::Utc::now().naive_utc();

    sharing::update(
        &mut state.db,
        &cook_and_run_id,
        &claims.sub,
        &payload.to(&Uuid::new_v4(), &time),
    )?;

    Ok(())
}

/// Get team sharing configuration
async fn get_share_config(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> Result<ShareTeamConfig, RestError> {
    let share = sharing::get_by_id(&mut state.db, &cook_and_run_id, &claims.sub)?;

    Ok(ShareTeamConfig::from(share))
}

/// Delete team sharing configuration
async fn delete_share_config(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> Result<(), RestError> {
    sharing::delete(&mut state.db, &cook_and_run_id, &claims.sub)?;
    Ok(())
}
