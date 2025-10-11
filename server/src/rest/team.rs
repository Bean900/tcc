use axum::{
    extract::{Path, Query, State},
    middleware::from_fn_with_state,
    response::{IntoResponse, Json, Response},
    routing::{delete, get, patch, post},
    Extension, Router,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::RestError,
    rest::{
        auth::{require_permission, Claims, CREATE_PERMISSION, READ_PERMISSION, UPDATE_PERMISSION},
        models::{PaginationInfo, Team, TeamCreateData, TeamUpdateData},
    },
    team, AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListTeamsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<TeamSortOption>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeamSortOption {
    NameAsc,
    NameDesc,
    CreatedAsc,
    CreatedDesc,
}

#[derive(Debug, Serialize)]
pub struct TeamListResponse {
    pub data: Vec<Team>,
    pub pagination: PaginationInfo,
}

impl IntoResponse for TeamListResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run/:cook_and_run_id/teams",
            get(list_teams).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            post(create_team).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            get(get_team).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            patch(update_team).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            delete(delete_team).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
}

/// List all teams for a cook and run project
async fn list_teams(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Query(params): Query<ListTeamsQuery>,
) -> Result<TeamListResponse, RestError> {
    let result: Vec<Team> = team::get_list(&mut state.db, &cook_and_run_id, &claims.sub)?
        .into_iter()
        .map(Team::from)
        .collect();

    let response = TeamListResponse {
        data: result,
        pagination: PaginationInfo::new(),
    };
    Ok(response)
}

/// Create team for cook and run project
async fn create_team(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<TeamCreateData>,
) -> Result<(), RestError> {
    let time = chrono::Utc::now().naive_utc();
    team::create(
        &mut state.db,
        &claims.sub,
        &payload.to(&cook_and_run_id, &team_id, &claims.sub, &time),
    )
}

/// Get team details
async fn get_team(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
) -> Result<Team, RestError> {
    let result = team::get(&mut state.db, &cook_and_run_id, &claims.sub, &team_id)?;
    Ok(Team::from(result))
}

/// Update team for cook and run project
async fn update_team(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<TeamUpdateData>,
) -> Result<(), RestError> {
    let time = chrono::Utc::now().naive_utc();
    team::update(
        &mut state.db,
        &claims.sub,
        &payload.to(&cook_and_run_id, &team_id, &claims.sub, &time),
    )
}

/// Delete team for cook and run project
async fn delete_team(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
) -> Result<(), RestError> {
    team::delete(&mut state.db, &cook_and_run_id, &claims.sub, &team_id)
}
