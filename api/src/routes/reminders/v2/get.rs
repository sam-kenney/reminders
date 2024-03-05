//! Get method
//!
//! This module contains the get method for the reminders API.
use crate::models::{reminder::Reminder, result::Result};
use crate::SharedState;
use axum::{
    extract::State,
    response::{self, IntoResponse, Response},
};
use serde_json::Value;
use std::collections::HashMap;

type RawReminder = HashMap<String, HashMap<String, Value>>;

/// Get all reminders.
///
/// # Returns
///
/// A JSON response with all reminders.
pub async fn get(State(state): State<SharedState>) -> Result<Response> {
    let mut db = state.read().await.db.clone();
    let data: RawReminder = db.get("reminders/v2").await?;

    Ok(response::Json(Reminder::from_json(data)).into_response())
}
