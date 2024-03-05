//! Result type for routes.
pub type Result<T> = std::result::Result<T, axum::response::Response>;
