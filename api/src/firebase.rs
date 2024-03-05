//! Interface with Firebase.
use axum::response::IntoResponse;
use gcp_auth::{AuthenticationManager, Token};

/// Errors that can occur when interfacing with Firebase.
#[derive(Debug)]
pub enum Error {
    URINotSet,
    Authentication,
    NotFound,
    PostData,
    DeleteData,
}

type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

/// Allow FirebaseError to be displayed.
impl std::fmt::Display for Error {
    /// Display FirebaseError.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::URINotSet => write!(f, "FIREBASE_URI not set"),
            Error::Authentication => write!(f, "Authentication error"),
            Error::NotFound => write!(f, "Not found"),
            Error::PostData => write!(f, "Error posting data"),
            Error::DeleteData => write!(f, "Error deleting data"),
        }
    }
}

impl std::convert::From<Error> for axum::response::Response {
    fn from(value: Error) -> Self {
        log::error!("{value}");
        crate::models::generic_response::ResponseMessage::from(value).into_response()
    }
}

/// Firebase interface.
#[derive(Clone)]
pub struct Firebase {
    token: Token,
    client: reqwest::Client,
    pub uri: String,
}

impl Firebase {
    /// Create a new Firebase instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the FIREBASE_URI environment variable is not set or if authentication fails.
    pub async fn new() -> Result<Self> {
        let uri = std::env::var("FIREBASE_URI").map_err(|_| Error::URINotSet)?;
        let token = Firebase::get_token().await?;

        Ok(Self {
            token,
            client: reqwest::Client::new(),
            uri,
        })
    }

    /// Get a Firebase token.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails.
    async fn get_token() -> Result<Token> {
        AuthenticationManager::new()
            .await
            .unwrap()
            .get_token(&[
                "https://www.googleapis.com/auth/firebase.database",
                "https://www.googleapis.com/auth/userinfo.email",
            ])
            .await
            .map_err(|_| Error::Authentication)
    }

    /// Refresh the Firebase token.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails.
    async fn refresh(&mut self) -> Result<()> {
        if self.token.has_expired() {
            self.token = Firebase::get_token().await?;
        }

        Ok(())
    }

    /// Get data from Firebase.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the data.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails or if the data is not found.
    pub async fn get(&mut self, path: &str) -> Result<reqwest::Response> {
        self.refresh().await?;

        let url = format!("{}{}.json", &self.uri, path);

        let response = self
            .client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
            .map_err(|_| Error::Authentication)?;

        match response.status().is_success() {
            true => Ok(response),
            false => Err(Error::NotFound),
        }
    }

    /// Post data to Firebase.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the data.
    /// * `data` - The data to post.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails or if the data is not found.
    pub async fn post<T>(&mut self, path: &str, data: T) -> Result<()>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        self.refresh().await?;

        let url = format!("{}{}.json", &self.uri, path);

        let response = self
            .client
            .post(url)
            .bearer_auth(self.token.as_str())
            .json(&data)
            .send()
            .await
            .map_err(|_| Error::Authentication)?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(Error::PostData),
        }
    }

    /// Update a record in Firebase.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the data.
    /// * `data` - The data to update.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails or if the data is not found.
    pub async fn put<T>(&mut self, path: &str, data: T) -> Result<()>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        self.refresh().await?;

        let url = format!("{}{}.json", &self.uri, path);

        let response = self
            .client
            .put(url)
            .bearer_auth(self.token.as_str())
            .json(&data)
            .send()
            .await
            .map_err(|_| Error::Authentication)?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(Error::PostData),
        }
    }

    /// Delete a record in Firebase.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the data.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails or if the data is not found.
    pub async fn delete(&mut self, path: &str) -> Result<()> {
        self.refresh().await?;

        let url = format!("{}{}.json", &self.uri, path);

        let response = self
            .client
            .delete(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
            .map_err(|_| Error::Authentication)?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(Error::DeleteData),
        }
    }
}
