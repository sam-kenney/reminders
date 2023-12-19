//! Post method
//!
//! This module contains the post method for the reminders API.
use crate::models::{generic_response::GenericResponse, reminder::Reminder};
use crate::SharedState;
use axum::{
    extract::{self, State},
    http,
    response::{self, IntoResponse, Response},
};

/// Create a new reminder.
///
/// # Returns
///
/// A JSON response with a 201 status code.
pub async fn post(
    State(state): State<SharedState>,
    extract::Json(reminder): extract::Json<Reminder>,
) -> Response {
    let mut db = state.write().await.db.clone();
    let resp = db.post("reminders", reminder).await;

    match resp {
        Ok(_) => http::response::Builder::new()
            .status(201)
            .body(
                response::Json(GenericResponse::new("Created reminder").to_json())
                    .into_response()
                    .into_body(),
            )
            .unwrap(),

        Err(e) => {
            log::error!("{:?}", e);
            let error = GenericResponse::from_string(e.to_string());
            response::Json(error).into_response()
        }
    }
}
