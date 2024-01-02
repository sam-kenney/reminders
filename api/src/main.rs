//! Main entry point for the API.
mod firebase;
mod middleware;
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

    let db = firebase.unwrap_or_else(|e| {
        log::error!("{:?}", e);
        std::process::exit(1);
    });

    let state = Arc::new(RwLock::new(AppState { db }));
    let app = Router::new()
        .fallback(routes::err_404::handle_404)
        .route("/reminders/v2/", routes::reminders::v2::router())
        .route_layer(axum::middleware::from_fn(middleware::auth::auth))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();

    log::info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap()
}
