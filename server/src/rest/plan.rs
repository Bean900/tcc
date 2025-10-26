use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    middleware::from_fn_with_state,
    response::{Json, Response},
    routing::{get, patch},
    Extension, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::RestError,
    rest::{
        auth::{require_permission, Claims, READ_PERMISSION, UPDATE_PERMISSION},
        models::Plan,
    },
    AppState,
};

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run/:cook_and_run_id/plan",
            get(get_event_plan).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/plan",
            patch(update_event_plan).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
}

/// Get complete event plan
async fn get_event_plan(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> Result<Plan, RestError> {
}

/// Update event plan
async fn update_event_plan(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<Plan>,
) -> Result<(), RestError> {
}
