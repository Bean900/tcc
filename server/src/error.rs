use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug)]
pub enum RestError {
    Unauthorized { message: String },
    BadRequest { message: String },
    InternalServer { message: String },
    NotFound { message: String },
    Conflict { message: String },
    Forbidden { message: String },
}

impl RestError {
    pub fn bad_request_error(message: &str) -> Self {
        RestError::BadRequest {
            message: message.to_string(),
        }
    }

    pub fn internal_server_error(message: &str) -> Self {
        RestError::InternalServer {
            message: message.to_string(),
        }
    }

    pub fn conflict_error(message: &str) -> Self {
        RestError::Conflict {
            message: message.to_string(),
        }
    }
}

#[derive(Serialize)]
struct ErrorBody {
    #[serde(skip_serializing)]
    status: StatusCode,
    error: String,
    message: String,
    timestamp: DateTime<Utc>,
}

impl ErrorBody {
    fn from_rest_error(rest_error: &RestError) -> Self {
        let (status, message) = match rest_error {
            RestError::BadRequest { message } => (StatusCode::BAD_REQUEST, message),
            RestError::InternalServer { message } => (StatusCode::INTERNAL_SERVER_ERROR, message),
            RestError::Conflict { message } => (StatusCode::CONFLICT, message),
            RestError::Unauthorized { message } => (StatusCode::UNAUTHORIZED, message),
            RestError::NotFound { message } => (StatusCode::NOT_FOUND, message),
            RestError::Forbidden { message } => (StatusCode::FORBIDDEN, message),
        };
        ErrorBody {
            status: status,
            error: status
                .canonical_reason()
                .expect("Expect status code to exists!")
                .to_string(),
            message: message.clone(),
            timestamp: Utc::now(),
        }
    }
}

impl IntoResponse for RestError {
    fn into_response(self) -> Response {
        let error_body = ErrorBody::from_rest_error(&self);
        (error_body.status, Json(error_body)).into_response()
    }
}

pub fn map_not_found_cook_and_run(
    cook_and_run_id: &Uuid,
    needed_for: &str,
    e: diesel::result::Error,
) -> RestError {
    warn!(
        "Could not find cook and run project with id {} for {}: {}",
        cook_and_run_id, needed_for, e
    );
    RestError::NotFound {
        message: "Entity not found".to_string(),
    }
}
