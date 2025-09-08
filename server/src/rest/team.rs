use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{Course, Team, TeamCreateData, TeamSummary, TeamUpdateData},
    ApiResult, AppState, PaginationInfo,
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

#[derive(Debug, Serialize)]
pub struct TeamPlanResponse {
    pub introduction: String,
    pub current_team: TeamSummary,
    pub walking_path: Vec<WalkingPathItem>,
}

#[derive(Debug, Serialize)]
pub struct WalkingPathItem {
    pub course: Course,
    pub team: TeamSummary,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cook_and_run/:cook_and_run_id/teams", get(list_teams))
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            post(create_team),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            get(get_team),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            patch(update_team),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id",
            delete(delete_team),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id/plan",
            get(get_team_plan),
        )
}

/// List all teams for a cook and run project
async fn list_teams(
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Query(params): Query<ListTeamsQuery>,
) -> ApiResult<Json<TeamListResponse>> {
    // TODO: Implement database query
    // Filter by cook_and_run_id
    // Apply pagination and sorting
    // Return 404 if cook_and_run_id doesn't exist

    let response = TeamListResponse {
        data: vec![],
        pagination: PaginationInfo {
            page: params.page.unwrap_or(1),
            limit: params.limit.unwrap_or(20),
            total: 0,
            total_pages: 0,
            has_next: false,
            has_prev: false,
        },
    };

    Ok(Json(response))
}

/// Create team for Cook and Run project
async fn create_team(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<TeamCreateData>,
) -> ApiResult<(StatusCode, Json<Team>)> {
    // TODO: Implement team creation logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id already exists -> 409 Conflict
    // Validate payload (email format, phone format, etc.) -> 422 Validation Error
    // Create team in database

    let team = Team {
        id: team_id,
        created_by_user: payload.user_id,
        name: payload.name,
        created: chrono::Utc::now(),
        edited: chrono::Utc::now(),
        address: payload.address,
        mail: payload.mail,
        phone: payload.phone,
        members: payload.members,
        diets: payload.diets,
        needs_check: payload.needs_check,
        status: crate::models::TeamStatus::Pending, // or Approved based on needs_check
        note_list: vec![],
    };

    Ok((StatusCode::CREATED, Json(team)))
}

/// Get team details
async fn get_team(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<Team>> {
    // TODO: Implement database lookup
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Return team details

    todo!("Implement get_team")
}

/// Update team for Cook and Run project
async fn update_team(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<TeamUpdateData>,
) -> ApiResult<Json<Team>> {
    // TODO: Implement team update logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Validate payload -> 422 Validation Error
    // Update team in database

    todo!("Implement update_team")
}

/// Delete team for cook and run project
async fn delete_team(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<StatusCode> {
    // TODO: Implement team deletion logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Check if team is assigned to host courses -> 400 Bad Request
    // Delete team from database

    Ok(StatusCode::NO_CONTENT)
}

/// Get personalized plan for team
async fn get_team_plan(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<TeamPlanResponse>> {
    // TODO: Implement personalized plan generation
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Generate walking plan based on team assignments and routes
    // Include hosting assignments and guest locations

    todo!("Implement get_team_plan")
}
