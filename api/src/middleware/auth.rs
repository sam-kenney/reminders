//! Basic authentication middleware using a shared secret.
use axum::{extract::Request, http, http::StatusCode, middleware::Next, response::Response};

/// Ensure a request contains the shared secret.
///
/// # Returns
///
/// A 401 response if the request does not contain the shared secret.
/// Otherwise, the next middleware is called.
pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    if auth_header.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !authorize_current_user(auth_header.unwrap()) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

/// Validate that the shared secret matches the one in the environment.
fn authorize_current_user(auth_token: &str) -> bool {
    let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set");

    format!("Bearer {}", token).as_str() == auth_token
}
