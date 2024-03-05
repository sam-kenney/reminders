//! Put method
//!
//! This module contains the put method for the reminders API.
use crate::models::{generic_response::ResponseMessage, reminder::Reminder, result::Result};
use crate::SharedState;
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Update a reminder.
///
/// # Returns
///
/// A JSON response with a 200 status code.
pub async fn put(
    State(state): State<SharedState>,
    extract::Json(mut reminder): extract::Json<Reminder>,
) -> Result<Response> {
    if reminder.id.is_none() {
        return Err(ResponseMessage::from("Reminder is missing the id field")
            .with_status(StatusCode::BAD_REQUEST)
            .into_response());
    }

    let mut db = state.write().await.db.clone();

    let path = format!("reminders/v2/{}", reminder.id.clone().unwrap());
    reminder.id = None;
    let resp = db.put(path.as_str(), reminder).await;

    Ok(resp.map(|_| ResponseMessage::from("Updated reminder").into_response())?)
}
