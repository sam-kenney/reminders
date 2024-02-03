//! Get method
//!
//! This module contains the get method for the reminders API.
use crate::models::{generic_response::GenericResponse, reminder::Reminder};
use crate::SharedState;
use axum::{
    extract::State,
    response::{self, IntoResponse, Response},
};
use serde_json::Value;
use std::collections::HashMap;

/// Get all reminders.
///
/// # Returns
///
/// A JSON response with all reminders.
pub async fn get(State(state): State<SharedState>) -> Response {
    let mut db = state.read().await.db.clone();
    let resp = db.get("reminders/v2").await;

    let res_json: Result<HashMap<String, HashMap<String, Value>>, reqwest::Error> = match resp {
        Ok(d) => d.json().await,
        Err(e) => {
            log::error!("{:?}", e);
            let error = GenericResponse::from_string(e.to_string());
            return response::Json(error).into_response();
        }
    };

    let data = match res_json {
        Ok(d) => d,
        Err(_) => {
            log::warn!("No reminders in firebase");
            return response::Json(vec![] as Vec<Reminder>).into_response();
        }
    };

    response::Json(Reminder::from_json(data)).into_response()
}
