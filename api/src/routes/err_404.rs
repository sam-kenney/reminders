//! # 404 Error handler
use crate::models::generic_response::GenericResponse;
use axum::http::Response;

/// Handle 404 errors.
///
/// # Returns
///
/// A JSON response with a 404 status code.
pub async fn handle_404() -> Response<String> {
    let error = GenericResponse::new("Not found");

    Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(error.to_json())
        .unwrap()
}
