//! Reminder model.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Reminder model
///
/// When serializing, the id field is skipped if it is None.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reminder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub due: u64,
}

impl Reminder {
    /// Create a new Reminder.
    ///
    /// # Arguments
    ///
    /// * `json` - A HashMap of HashMaps of Values.
    ///
    /// # Returns
    ///
    /// A Vec of Reminders.
    pub fn from_json(json: HashMap<String, HashMap<String, Value>>) -> Vec<Reminder> {
        json.into_iter()
            .map(|(id, content)| Reminder {
                id: Some(id),
                title: content["title"].as_str().unwrap_or("").to_string(),
                due: content["due"].as_u64().unwrap_or(0),
            })
            .collect()
    }
}
