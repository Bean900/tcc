use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug)]
pub enum RestError {
    BadRequest { message: String },
    InternalServer { message: String },
    Conflict { message: String },
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
