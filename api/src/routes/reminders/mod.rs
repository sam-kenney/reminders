//! Reminders endpoint routing.
mod delete;
mod get;
mod post;
mod put;
use crate::AppState;
use axum::routing::MethodRouter;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Returns a router with all the request methods for the reminders.
/// This is the entry point for the reminders routes.
pub fn router() -> MethodRouter<Arc<RwLock<AppState>>> {
    axum::routing::get(self::get::get)
        .post(self::post::post)
        .put(self::put::put)
        .delete(self::delete::delete)
}
