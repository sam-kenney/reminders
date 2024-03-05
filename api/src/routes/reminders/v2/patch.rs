//! Patch method
//!
//! This module contains the patch method for the reminders API.
use crate::models::{generic_response::ResponseMessage, reminder::Reminder, result::Result};
use crate::SharedState;
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Bulk update reminders.
///
/// # Returns
///
/// A JSON response with a 200 status code.
pub async fn patch(
    State(state): State<SharedState>,
    extract::Json(reminders): extract::Json<Vec<Reminder>>,
) -> Result<Response> {
    let errors: Vec<Reminder> = reminders
        .clone()
        .into_iter()
        .filter(move |r| r.id.is_none())
        .collect();

    if errors.len() > 1 {
        return Err(ResponseMessage::from("Reminder is missing id field")
            .with_status(StatusCode::BAD_REQUEST)
            .into_response());
    }

    let mut db = state.write().await.db.clone();

    let data = crate::models::reminder::reminders_to_firebase(reminders);
    let resp = db.put("reminders/v2", data).await;

    Ok(resp.map(|_| ResponseMessage::from("Updated reminder").into_response())?)
}
