use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::{Note, NoteCreateData},
    ApiResult, AppState,
};

#[derive(Debug, Deserialize)]
pub struct ListNotesQuery {
    pub sort: Option<NoteSortOption>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteSortOption {
    CreatedAsc,
    CreatedDesc,
}

#[derive(Debug, Serialize)]
pub struct NoteListResponse {
    pub data: Vec<Note>,
    pub count: usize,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id/notes",
            get(get_team_notes),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id/note/:note_id",
            post(create_team_note),
        )
        .route(
            "/cook_and_run/:cook_and_run_id/team/:team_id/note/:note_id",
            delete(delete_team_note),
        )
}

/// Get all notes for a team
async fn get_team_notes(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id)): Path<(Uuid, Uuid)>,
    Query(params): Query<ListNotesQuery>,
) -> ApiResult<Json<NoteListResponse>> {
    // TODO: Implement database query
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Filter notes by team_id
    // Apply sorting based on params.sort (default: created_desc)

    let response = NoteListResponse {
        data: vec![],
        count: 0,
    };

    Ok(Json(response))
}

/// Create note for team
async fn create_team_note(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id, note_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(payload): Json<NoteCreateData>,
) -> ApiResult<(StatusCode, Json<Note>)> {
    // TODO: Implement note creation logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Check if note_id already exists -> 409 Conflict
    // Validate payload (headline and content length) -> 422 Validation Error
    // Create note in database

    let note = Note {
        id: note_id,
        headline: payload.headline,
        content: payload.content,
        created: chrono::Utc::now(),
    };

    Ok((StatusCode::CREATED, Json(note)))
}

/// Delete note for team
async fn delete_team_note(
    State(state): State<AppState>,
    Path((cook_and_run_id, team_id, note_id)): Path<(Uuid, Uuid, Uuid)>,
) -> ApiResult<StatusCode> {
    // TODO: Implement note deletion logic
    // Check if cook_and_run_id exists -> 404 Not Found
    // Check if team_id exists -> 404 Not Found
    // Check if note_id exists -> 404 Not Found
    // Delete note from database

    Ok(StatusCode::NO_CONTENT)
}
