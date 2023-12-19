//! Main entry point for the API.
mod firebase;
mod models;
mod routes;
mod util;
use axum::Router;
use firebase::Firebase;
use std::sync::Arc;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<AppState>>;

/// Application state.
#[derive(Clone)]
pub struct AppState {
    db: Firebase,
}

/// Entry point.
#[tokio::main]
async fn main() {
    util::init_logger();

    let firebase = Firebase::new().await;

    let db = match firebase {
        Ok(db) => db,
        Err(e) => {
            log::error!("{e}");
            return;
        }
    };

    let state = Arc::new(RwLock::new(AppState { db }));
    let app = Router::new()
        .route("/reminders/", routes::reminders::router())
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let app = app.fallback(routes::err_404::handle_404);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();

    log::info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}
