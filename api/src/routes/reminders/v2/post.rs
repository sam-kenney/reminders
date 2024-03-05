//! Post method
//!
//! This module contains the post method for the reminders API.
use crate::models::{generic_response::ResponseMessage, reminder::Reminder, result::Result};
use crate::SharedState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Create a new reminder.
///
/// # Returns
///
/// A JSON response with a 201 status code.
pub async fn post(
    State(state): State<SharedState>,
    Json(reminder): Json<Reminder>,
) -> Result<Response> {
    let mut db = state.write().await.db.clone();
    let resp = db.post("reminders/v2", reminder).await;

    Ok(resp.map(|_| {
        ResponseMessage::from("Created reminder")
            .with_status(StatusCode::CREATED)
            .into_response()
    })?)
}
