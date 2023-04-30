use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    MissingCredentials,
    InternalServerError,
    UserAlreadyExists,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "missing credentials"),
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }

            Self::UserAlreadyExists => (StatusCode::CONFLICT, "user already exists"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
