//! Delete method
//!
//! This module contains the delete method for the reminders API.
use crate::models::{generic_response::GenericResponse, reminder::Reminder};
use crate::SharedState;
use axum::{
    extract::{self, State},
    http,
    response::{self, IntoResponse, Response},
};

/// Delete a reminder.
///
/// # Returns
///
/// A JSON response with a 200 response.
pub async fn delete(
    State(state): State<SharedState>,
    extract::Json(reminder): extract::Json<Reminder>,
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

    let resp = db.delete(path.as_str()).await;

    match resp {
        Ok(_) => response::Json(GenericResponse::new("Deleted reminder")).into_response(),
        Err(e) => {
            log::error!("{:?}", e);
            let error = GenericResponse::from_string(e.to_string());
            response::Json(error).into_response()
        }
    }
}
