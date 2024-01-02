//! Interface with Firebase.
use gcp_auth::{AuthenticationManager, Token};

/// Errors that can occur when interfacing with Firebase.
#[derive(Debug)]
pub enum FirebaseError {
    URINotSet,
    Authentication,
    NotFound,
    PostData,
    DeleteData,
}

/// Allow FirebaseError to be displayed.
impl std::fmt::Display for FirebaseError {
    /// Display FirebaseError.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirebaseError::URINotSet => write!(f, "FIREBASE_URI not set"),
            FirebaseError::Authentication => write!(f, "Authentication error"),
            FirebaseError::NotFound => write!(f, "Not found"),
            FirebaseError::PostData => write!(f, "Error posting data"),
            FirebaseError::DeleteData => write!(f, "Error deleting data"),
        }
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
    pub async fn new() -> Result<Self, FirebaseError> {
        let uri = std::env::var("FIREBASE_URI").map_err(|_| FirebaseError::URINotSet)?;
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
    async fn get_token() -> Result<Token, FirebaseError> {
        AuthenticationManager::new()
            .await
            .unwrap()
            .get_token(&[
                "https://www.googleapis.com/auth/firebase.database",
                "https://www.googleapis.com/auth/userinfo.email",
            ])
            .await
            .map_err(|_| FirebaseError::Authentication)
    }

    /// Refresh the Firebase token.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails.
    async fn refresh(&mut self) -> Result<(), FirebaseError> {
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
    pub async fn get(&mut self, path: &str) -> Result<reqwest::Response, FirebaseError> {
        self.refresh().await?;

        let url = format!("{}{}.json", &self.uri, path);

        let response = self
            .client
            .get(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
            .map_err(|_| FirebaseError::Authentication)?;

        match response.status().is_success() {
            true => Ok(response),
            false => Err(FirebaseError::NotFound),
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
    pub async fn post<T>(&mut self, path: &str, data: T) -> Result<(), FirebaseError>
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
            .map_err(|_| FirebaseError::Authentication)?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(FirebaseError::PostData),
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
    pub async fn put<T>(&mut self, path: &str, data: T) -> Result<(), FirebaseError>
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
            .map_err(|_| FirebaseError::Authentication)?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(FirebaseError::PostData),
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
    pub async fn delete(&mut self, path: &str) -> Result<(), FirebaseError> {
        self.refresh().await?;

        let url = format!("{}{}.json", &self.uri, path);

        let response = self
            .client
            .delete(url)
            .bearer_auth(self.token.as_str())
            .send()
            .await
            .map_err(|_| FirebaseError::Authentication)?;

        match response.status().is_success() {
            true => Ok(()),
            false => Err(FirebaseError::DeleteData),
        }
    }
}
