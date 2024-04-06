//! Main entry point for the API.
mod firebase;
mod logger;
mod middleware;
mod models;
mod routes;
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

async fn serve() -> Result<(), String> {
    let db = Firebase::new().await.map_err(|e| e.to_string())?;
    let state = Arc::new(RwLock::new(AppState { db }));

    let app = Router::new()
        .fallback(routes::err_404::handle_404)
        .route("/reminders/v2/", routes::reminders::v2::router())
        .route_layer(axum::middleware::from_fn(middleware::auth::auth))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.map_err(|e| e.to_string())?;

    log::info!("listening on http://{}", listener.local_addr().map_err(|e| e.to_string())?);

    axum::serve(listener, app).await.map_err(|e| e.to_string())
}

/// Entry point.
#[tokio::main]
async fn main() {
    logger::init();

    if let Err(e) = serve().await {
        log::error!("{e}");
        std::process::exit(1);
    }
}
