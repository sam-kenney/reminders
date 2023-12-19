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

impl std::cmp::PartialEq for Reminder {
    /// Validate whether a Reminder is equal to another Reminder.
    ///
    /// # Arguments
    ///
    /// * `other` - Another Reminder to compare to.
    ///
    /// # Returns
    ///
    /// A boolean.
    fn eq(&self, other: &Self) -> bool {
        if (self.id == other.id) && (self.title == other.title) && (self.due == other.due) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::models::reminder::Reminder;
    use std::collections::HashMap;

    /// Test whether the `Reminder::from_json` correctly interprets a Firebase response.
    #[test]
    fn test_from_json() {
        let mut outer: HashMap<String, HashMap<String, serde_json::Value>> = HashMap::new();
        let mut inner: HashMap<String, serde_json::Value> = HashMap::new();

        inner.insert("title".into(), "hello, world".into());
        inner.insert("due".into(), 1234.into());
        outer.insert("abc".into(), inner);

        let r = vec![Reminder {
            title: "hello, world".into(),
            due: 1234,
            id: Some("abc".into()),
        }];

        assert_eq!(r, Reminder::from_json(outer))
    }

    /// Test that a Reminder serialises properly when no ID is set.
    #[test]
    fn test_serialising_with_no_id() {
        let r = Reminder {
            title: "hello, world".into(),
            due: 1234,
            id: None,
        };

        let json = serde_json::to_string(&r).unwrap();

        assert_eq!(json, r#"{"title":"hello, world","due":1234}"#)
    }

    /// Test serialising a Reminder with an ID.
    #[test]
    fn test_serialising_with_id() {
        let r = Reminder {
            title: "hello, world".into(),
            due: 1234,
            id: Some("asdf".into()),
        };

        let json = serde_json::to_string(&r).unwrap();

        assert_eq!(json, r#"{"id":"asdf","title":"hello, world","due":1234}"#)
    }
}
