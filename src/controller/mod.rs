use axum::http::StatusCode;
use axum::response::{IntoResponse};

pub mod graphql;
mod user;

// TODO: use this error crate
enum ControllerError {
    InvalidContentType,
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            ControllerError::InvalidContentType => {
                (StatusCode::BAD_REQUEST, "Invalid content Type").into_response()
            }
        }
    }
}
