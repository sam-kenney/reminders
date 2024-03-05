//! Delete method
//!
//! This module contains the delete method for the reminders API.
use crate::{
    models::{generic_response::ResponseMessage, reminder::Reminder, result::Result},
    SharedState,
};
use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Delete a reminder.
///
/// # Returns
///
/// A JSON response with a 200 response.
pub async fn delete(
    State(state): State<SharedState>,
    extract::Json(reminder): extract::Json<Reminder>,
) -> Result<Response> {
    if reminder.id.is_none() {
        return Err(ResponseMessage::from("Reminder is missing the id field")
            .with_status(StatusCode::BAD_REQUEST)
            .into_response());
    }

    let mut db = state.write().await.db.clone();

    let path = format!("reminders/v2/{}", reminder.id.clone().unwrap());
    let resp = db.delete(path.as_str()).await;

    Ok(resp.map(|_| ResponseMessage::from("Deleted reminder").into_response())?)
}
