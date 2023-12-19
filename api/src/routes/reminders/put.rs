//! Put method
//!
//! This module contains the put method for the reminders API.
use crate::models::{generic_response::GenericResponse, reminder::Reminder};
use crate::SharedState;
use axum::{
    extract::{self, State},
    http,
    response::{self, IntoResponse, Response},
};

// Update a reminder.
///
/// # Returns
///
/// A JSON response with a 200 status code.
pub async fn put(
    State(state): State<SharedState>,
    extract::Json(mut reminder): extract::Json<Reminder>,
) -> Response {
    if reminder.id.is_none() {
        let error = GenericResponse::new("Missing reminder id");
        return http::response::Builder::new()
            .status(400)
            .body(response::Json(error).into_response().into_body())
            .unwrap();
    }

    let mut db = state.write().await.db.clone();

    let path = format!("reminders/{}", reminder.id.clone().unwrap());
    reminder.id = None;
    let resp = db.put(path.as_str(), reminder).await;

    match resp {
        Ok(_) => response::Json(GenericResponse::new("Updated reminder")).into_response(),
        Err(e) => {
            log::error!("{:?}", e);
            let error = GenericResponse::from_string(e.to_string());
            response::Json(error).into_response()
        }
    }
}
