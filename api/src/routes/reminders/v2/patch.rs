//! Patch method
//!
//! This module contains the patch method for the reminders API.
use crate::models::{generic_response::GenericResponse, reminder::Reminder};
use crate::SharedState;
use axum::{
    extract::{self, State},
    http::{self, StatusCode},
    response::{self, IntoResponse, Response},
};

/// Bulk update reminders.
///
/// # Returns
///
/// A JSON response with a 200 status code.
pub async fn patch(
    State(state): State<SharedState>,
    extract::Json(reminders): extract::Json<Vec<Reminder>>,
) -> Response {
    let errors: Vec<Reminder> = reminders
        .clone()
        .into_iter()
        .filter(move |r| r.id.is_none())
        .collect();

    if errors.len() > 1 {
        let error = GenericResponse::new("Missing reminder id");
        return http::response::Builder::new()
            .status(StatusCode::BAD_REQUEST)
            .body(response::Json(error).into_response().into_body())
            .unwrap();
    }

    let mut db = state.write().await.db.clone();

    let data = crate::models::reminder::reminders_to_firebase(reminders);

    let resp = db.put("reminders/v2", data).await;

    match resp {
        Ok(_) => response::Json(GenericResponse::new("Updated reminder")).into_response(),
        Err(e) => {
            log::error!("{:?}", e);
            let error = GenericResponse::from_string(e.to_string());
            response::Json(error).into_response()
        }
    }
}
