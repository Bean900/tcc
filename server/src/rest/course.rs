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
    course::{self},
    error::RestError,
    rest::{
        auth::{require_permission, Claims, READ_PERMISSION, UPDATE_PERMISSION},
        models::{Course, CourseCreateData, CourseUpdateData},
    },
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListCoursesQuery {
    pub sort: Option<CourseSortOption>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseSortOption {
    TimeAsc,
    TimeDesc,
}

#[derive(Debug, Serialize)]
pub struct CourseListResponse {
    pub data: Vec<Course>,
    pub count: usize,
}

impl IntoResponse for CourseListResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run/:cook_and_run_id/courses",
            get(list_courses).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/course/:course_id",
            post(create_course).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/course/:course_id",
            get(get_course).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/course/:course_id",
            patch(update_course).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/course/:course_id",
            delete(delete_course).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
}

/// List all courses for a cook and run project
async fn list_courses(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Query(params): Query<ListCoursesQuery>,
) -> Result<CourseListResponse, RestError> {
    let result: Vec<Course> = course::get_list(&mut state.db, &cook_and_run_id, &claims.sub)?
        .into_iter()
        .map(Course::from)
        .collect();

    let count = result.len();

    let response = CourseListResponse {
        data: result,
        count,
    };
    Ok(response)
}

/// Create course for cook and run project
async fn create_course(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, course_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CourseCreateData>,
) -> Result<(), RestError> {
    course::create(
        &mut state.db,
        &claims.sub,
        &payload.to(&cook_and_run_id, &course_id),
    )
}

/// Get course details
async fn get_course(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, course_id)): Path<(Uuid, Uuid)>,
) -> Result<Course, RestError> {
    let result = course::get(&mut state.db, &cook_and_run_id, &claims.sub, &course_id)?;
    Ok(Course::from(result))
}

/// Update course for cook and run project
async fn update_course(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, course_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CourseUpdateData>,
) -> Result<(), RestError> {
    course::update(
        &mut state.db,
        &claims.sub,
        &payload.to(&cook_and_run_id, &course_id),
    )
}

/// Delete course for cook and run project
async fn delete_course(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path((cook_and_run_id, course_id)): Path<(Uuid, Uuid)>,
) -> Result<(), RestError> {
    course::delete(&mut state.db, &cook_and_run_id, &claims.sub, &course_id)
}
