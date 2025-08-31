use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::{IntoResponse, Json, Response},
    routing::{delete, get, patch, post},
    Extension, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    cook_and_run::{
        create_cook_and_run, delete_cook_and_run, get_cook_and_run, get_list_of_cook_and_run_meta,
        update_cook_and_run_name,
    },
    error::RestError,
    rest::{
        auth::{
            is_user_authenticated, require_permission, AuthUser, AuthenticatedUser, Claims,
            CREATE_PERMISSION, DELETE_PERMISSION, READ_PERMISSION, UPDATE_PERMISSION,
        },
        models::{Address, CookAndRun, CookAndRunCreateData, CookAndRunMeta, PaginationInfo},
    },
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListCookAndRunQuery {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort: Option<SortOption>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortOption {
    CreatedAsc,
    CreatedDesc,
    NameAsc,
    NameDesc,
    EditedAsc,
    EditedDesc,
}

#[derive(Debug, Serialize)]
pub struct CookAndRunListResponse {
    pub data: Vec<CookAndRunMeta>,
    pub pagination: PaginationInfo,
}

impl IntoResponse for CookAndRunListResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl AuthenticatedUser for CookAndRunListResponse {
    fn user_id(&self) -> AuthUser {
        AuthUser::AllOf(self.data.iter().map(|item| item.user_id.clone()).collect())
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateNameRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateNameResponse {
    pub id: Uuid,
    pub name: String,
    pub edited: chrono::DateTime<chrono::Utc>,
}

pub fn routes(app_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run",
            get(list_cook_and_run_projects).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id",
            post(create_cook_and_run_project).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(CREATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id",
            get(get_cook_and_run_project).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(READ_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id",
            delete(delete_cook_and_run_project).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(DELETE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/name",
            patch(patch_cook_and_run_name).layer(from_fn_with_state(
                app_state.clone(),
                require_permission(UPDATE_PERMISSION),
            )),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/start_point",
            patch(update_start_point),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/end_point",
            patch(update_end_point),
        )
}

/// List all cook and run projects
async fn list_cook_and_run_projects(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Query(params): Query<ListCookAndRunQuery>,
) -> Result<CookAndRunListResponse, RestError> {
    let result: Vec<CookAndRunMeta> =
        get_list_of_cook_and_run_meta(&mut state.db, &params.user_id)?
            .iter()
            .map(CookAndRunMeta::from)
            .collect();

    let len = result.len();

    let response = CookAndRunListResponse {
        data: result,
        pagination: PaginationInfo {
            page: params.page.unwrap_or(1),
            limit: params.limit.unwrap_or(20),
            total: len as u64,
            total_pages: 0,
            has_next: false,
            has_prev: false,
        },
    };

    is_user_authenticated(&response, &claims)?;

    Ok(response)
}

/// Create a new cook and run project
async fn create_cook_and_run_project(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<CookAndRunCreateData>,
) -> Result<CookAndRun, RestError> {
    is_user_authenticated(&payload, &claims)?;

    let time = chrono::Utc::now().naive_utc();

    let result = create_cook_and_run(
        &mut state.db,
        payload.to_cook_and_run_create(&cook_and_run_id, &time),
    )?;

    Ok(CookAndRun::from(result))
}

/// Get cook and run project details
async fn get_cook_and_run_project(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> Result<CookAndRun, RestError> {
    let result = get_cook_and_run(&mut state.db, &cook_and_run_id, &claims.sub)?;
    Ok(CookAndRun::from(result))
}

/// Delete cook and run project
async fn delete_cook_and_run_project(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
) -> Result<(), RestError> {
    delete_cook_and_run(&mut state.db, &cook_and_run_id, &claims.sub)
}

/// Update cook and run project name
async fn patch_cook_and_run_name(
    Extension(claims): Extension<Claims>,
    State(mut state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<UpdateNameRequest>,
) -> Result<(), RestError> {
    update_cook_and_run_name(
        &mut state.db,
        &cook_and_run_id,
        &claims.sub,
        payload.name.as_str(),
    )
}

/// Update start point
async fn update_start_point(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<Address>,
) -> Result<CookAndRunListResponse, RestError> {
    todo!();
}

/// Update end point
async fn update_end_point(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(cook_and_run_id): Path<Uuid>,
    Json(payload): Json<Address>,
) -> Result<CookAndRunListResponse, RestError> {
    todo!();
}
