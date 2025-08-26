use axum::{routing::{get, post, put, patch, delete}, Router, Json};
use axum::http::StatusCode;
use crate::state::AppState;
use crate::error::{AppResult};
use std::sync::Arc;

pub fn build_router(state: Arc<AppState>) -> Router {
    let mut app = Router::new();
    app = app.route("/cook_and_run", get(get_cook_and_run));
    app = app.route("/cook_and_run/{cookAndRunId}", post(post_cook_and_run_cookandrunid));
    app = app.route("/cook_and_run/{cookAndRunId}", get(get_cook_and_run_cookandrunid));
    app = app.route("/cook_and_run/{cookAndRunId}", delete(delete_cook_and_run_cookandrunid));
    app = app.route("/cook_and_run/{cookAndRunId}/name", patch(patch_cook_and_run_cookandrunid_name));
    app = app.route("/cook_and_run/{cookAndRunId}/start_point", patch(patch_cook_and_run_cookandrunid_start_point));
    app = app.route("/cook_and_run/{cookAndRunId}/end_point", patch(patch_cook_and_run_cookandrunid_end_point));
    app = app.route("/cook_and_run/{cookAndRunId}/courses", get(get_cook_and_run_cookandrunid_courses));
    app = app.route("/cook_and_run/{cookAndRunId}/course/{courseId}", post(post_cook_and_run_cookandrunid_course_courseid));
    app = app.route("/cook_and_run/{cookAndRunId}/course/{courseId}", get(get_cook_and_run_cookandrunid_course_courseid));
    app = app.route("/cook_and_run/{cookAndRunId}/course/{courseId}", patch(patch_cook_and_run_cookandrunid_course_courseid));
    app = app.route("/cook_and_run/{cookAndRunId}/course/{courseId}", delete(delete_cook_and_run_cookandrunid_course_courseid));
    app = app.route("/cook_and_run/{cookAndRunId}/course/{courseId}/hosts", patch(patch_cook_and_run_cookandrunid_course_courseid_hosts));
    app = app.route("/cook_and_run/{cookAndRunId}/teams", get(get_cook_and_run_cookandrunid_teams));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}", post(post_cook_and_run_cookandrunid_team_teamid));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}", get(get_cook_and_run_cookandrunid_team_teamid));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}", patch(patch_cook_and_run_cookandrunid_team_teamid));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}", delete(delete_cook_and_run_cookandrunid_team_teamid));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}/plan", get(get_cook_and_run_cookandrunid_team_teamid_plan));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}/notes", get(get_cook_and_run_cookandrunid_team_teamid_notes));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}/note/{noteId}", post(post_cook_and_run_cookandrunid_team_teamid_note_noteid));
    app = app.route("/cook_and_run/{cookAndRunId}/team/{teamId}/note/{noteId}", delete(delete_cook_and_run_cookandrunid_team_teamid_note_noteid));
    app = app.route("/cook_and_run/{cookAndRunId}/share_team_config", patch(patch_cook_and_run_cookandrunid_share_team_config));
    app = app.route("/cook_and_run/{cookAndRunId}/share_team_config", get(get_cook_and_run_cookandrunid_share_team_config));
    app = app.route("/cook_and_run/{cookAndRunId}/share_team_config", delete(delete_cook_and_run_cookandrunid_share_team_config));
    app = app.route("/cook_and_run/{cookAndRunId}/plan", get(get_cook_and_run_cookandrunid_plan));
    app = app.route("/cook_and_run/{cookAndRunId}/plan", patch(patch_cook_and_run_cookandrunid_plan));
    app = app.route("/health", get(get_health));
    app.with_state(state)
}

type State = std::sync::Arc<AppState>;


pub async fn get_cook_and_run(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run" 
    }))))
}


pub async fn post_cook_and_run_cookandrunid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "post_cook_and_run_cookandrunid" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid" 
    }))))
}


pub async fn delete_cook_and_run_cookandrunid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "delete_cook_and_run_cookandrunid" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_name(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_name" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_start_point(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_start_point" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_end_point(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_end_point" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_courses(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_courses" 
    }))))
}


pub async fn post_cook_and_run_cookandrunid_course_courseid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "post_cook_and_run_cookandrunid_course_courseid" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_course_courseid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_course_courseid" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_course_courseid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_course_courseid" 
    }))))
}


pub async fn delete_cook_and_run_cookandrunid_course_courseid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "delete_cook_and_run_cookandrunid_course_courseid" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_course_courseid_hosts(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_course_courseid_hosts" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_teams(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_teams" 
    }))))
}


pub async fn post_cook_and_run_cookandrunid_team_teamid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "post_cook_and_run_cookandrunid_team_teamid" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_team_teamid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_team_teamid" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_team_teamid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_team_teamid" 
    }))))
}


pub async fn delete_cook_and_run_cookandrunid_team_teamid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "delete_cook_and_run_cookandrunid_team_teamid" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_team_teamid_plan(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_team_teamid_plan" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_team_teamid_notes(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_team_teamid_notes" 
    }))))
}


pub async fn post_cook_and_run_cookandrunid_team_teamid_note_noteid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "post_cook_and_run_cookandrunid_team_teamid_note_noteid" 
    }))))
}


pub async fn delete_cook_and_run_cookandrunid_team_teamid_note_noteid(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "delete_cook_and_run_cookandrunid_team_teamid_note_noteid" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_share_team_config(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_share_team_config" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_share_team_config(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_share_team_config" 
    }))))
}


pub async fn delete_cook_and_run_cookandrunid_share_team_config(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "delete_cook_and_run_cookandrunid_share_team_config" 
    }))))
}


pub async fn get_cook_and_run_cookandrunid_plan(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_cook_and_run_cookandrunid_plan" 
    }))))
}


pub async fn patch_cook_and_run_cookandrunid_plan(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "patch_cook_and_run_cookandrunid_plan" 
    }))))
}


pub async fn get_health(_state: axum::extract::State<State>) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({ 
        "error": "Not Implemented", 
        "operation": "get_health" 
    }))))
}

