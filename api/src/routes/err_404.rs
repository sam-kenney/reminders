//! # 404 Error handler
use crate::models::generic_response::ResponseMessage;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Handle 404 errors.
///
/// # Returns
///
/// A JSON response with a 404 status code.
pub async fn handle_404() -> Response {
    ResponseMessage::from("Not found")
        .with_status(StatusCode::NOT_FOUND)
        .into_response()
}
