use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{RequiredField, ShareTeamConfig},
    ApiResult, AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateShareConfigRequest {
    pub invite_text: String,
    pub needs_login: bool,
    pub default_needs_check: bool,
    pub required_fields: Vec<RequiredField>,
    pub max_teams: Option<u32>,
    pub registration_deadline: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ShareConfigResponse {
    #[serde(flatten)]
    pub config: ShareTeamConfig,
    pub share_url: String,
    pub registration_count: Option<u32>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            patch(create_share_config),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            get(get_share_config),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/share_team_config",
            delete(delete_share_config),
        )
}

/// Create team sharing configuration
async fn create_share_config(
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<CreateShareConfigRequest>,
) -> ApiResult<(StatusCode, Json<ShareConfigResponse>)> {
    // TODO: Implement share config creation logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if share config already exists -> 409 Conflict
    // Validate payload -> 422 Validation Error
    //   - invite_text length (1-1000 chars)
    //   - required_fields unique and valid enum values
    //   - max_teams range (1-100)
    //   - registration_deadline in future
    // Generate unique share token
    // Create config in database
    // Generate share URL

    let config = ShareTeamConfig {
        id: Uuid::new_v4(),
        invite_text: payload.invite_text,
        needs_login: payload.needs_login,
        default_needs_check: payload.default_needs_check,
        required_fields: payload.required_fields,
        max_teams: payload.max_teams,
        registration_deadline: payload.registration_deadline,
        created: chrono::Utc::now(),
    };

    let response = ShareConfigResponse {
        share_url: format!("https://app.travelingcook.com/register/{}", config.id),
        registration_count: Some(0),
        config,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Get team sharing configuration
async fn get_share_config(
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> ApiResult<Json<ShareConfigResponse>> {
    // TODO: Implement share config lookup
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if share config exists -> 404 Not Found
    // Get registration count from database
    // Generate share URL

    todo!("Implement get_share_config")
}

/// Delete team sharing configuration
async fn delete_share_config(
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    // TODO: Implement share config deletion logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if share config exists -> 404 Not Found
    // Delete config from database
    // This will disable the shared registration link

    Ok(StatusCode::NO_CONTENT)
}
