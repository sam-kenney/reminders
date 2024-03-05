//! Generic response model for returning JSON responses.
use axum::http::StatusCode;
use serde::Serialize;

/// Generic response model for returning JSON responses.
#[derive(Clone, Serialize)]
pub struct ResponseMessage {
    message: String,
    #[serde(skip_serializing)]
    status: StatusCode,
}

impl ResponseMessage {
    /// Set the status code for the response.
    pub fn with_status(&mut self, status: StatusCode) -> Self {
        self.status = status;
        self.clone()
    }

    /// Convert the response to a JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("ResponseMessage deserialized into invalid JSON")
    }
}

impl std::convert::From<String> for ResponseMessage {
    fn from(value: String) -> Self {
        Self {
            message: value,
            status: StatusCode::OK,
        }
    }
}

impl std::convert::From<&str> for ResponseMessage {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
            status: StatusCode::OK,
        }
    }
}

impl std::convert::From<crate::firebase::Error> for ResponseMessage {
    fn from(value: crate::firebase::Error) -> Self {
        Self {
            message: value.to_string(),
            status: StatusCode::OK,
        }
    }
}

impl axum::response::IntoResponse for ResponseMessage {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(self.status)
            .header("Content-Type", "application/json")
            .body(self.to_json().into_response().into_body())
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::models::generic_response::ResponseMessage;
    /// Test the `ResponseMessage.to_json` method.
    #[test]
    fn test_to_json() {
        let r = ResponseMessage::from("something happened");

        assert_eq!(*r.to_json(), *r#"{"message":"something happened"}"#)
    }
}
