use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{Json, Response},
    routing::{get, patch},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::Plan, ApiResult, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cook_and_run/:cook_and_run_id/plan", get(get_event_plan))
        .route(
            "/cook_and_run/:cook_and_run_id/plan",
            patch(update_event_plan),
        )
}

/// Get complete event plan
async fn get_event_plan(
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> ApiResult<Response> {
    // TODO: Implement event plan generation
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check user permissions -> 403 Forbidden
    // Generate complete event plan including:
    //   - Team assignments for each course
    //   - Walking routes between locations
    //   - Timing and logistics information
    // Support both JSON and PDF output based on Accept header

    // For now, return JSON response
    let plan = Plan {
        access: vec!["link".to_string(), "account".to_string()],
        introduction: "Welcome to our Cook & Run event! Follow your personalized plan below."
            .to_string(),
        hosting_assignments: vec![],
        walking_paths: std::collections::HashMap::new(),
    };

    let json_response = Json(plan);

    // TODO: Check Accept header for PDF vs JSON
    // If PDF requested, generate PDF and return with appropriate content-type

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&json_response.0).unwrap().into())
        .unwrap())
}

/// Update event plan
async fn update_event_plan(
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<Plan>,
) -> ApiResult<Json<Plan>> {
    // TODO: Implement event plan update logic
    // Check if cook_and_run
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&json_response.0).unwrap().into())
        .unwrap())
}
