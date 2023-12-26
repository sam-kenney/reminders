//! Utility functions for the server.
use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

/// Initialise logging on inbound requests and outgoing responses.
pub fn init_logger() {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::INFO);

    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();
}
