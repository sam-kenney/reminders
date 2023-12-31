//! Generic response model for returning JSON responses.
use serde::Serialize;

/// Generic response model for returning JSON responses.
#[derive(Serialize)]
pub struct GenericResponse {
    message: String,
}

impl GenericResponse {
    /// Create a new GenericResponse.
    ///
    /// # Arguments
    ///
    /// * `message` - A message to return in the response.
    pub fn new(message: &'static str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    /// Create a new GenericResponse from a String.
    ///
    /// # Arguments
    ///
    /// * `message` - A message to return in the response.
    pub fn from_string(message: String) -> Self {
        Self { message }
    }

    /// Convert the GenericResponse to a JSON string.
    ///
    /// # Returns
    ///
    /// A JSON string.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::generic_response::GenericResponse;
    /// Test the `GenericResponse.to_json` method.
    #[test]
    fn test_to_json() {
        let r = GenericResponse::new("something happened");

        assert_eq!(*r.to_json(), *r#"{"message":"something happened"}"#)
    }
}
