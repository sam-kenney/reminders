//! Reminder model.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Return a string with the first letter capitalised.
fn fix_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                return c.to_uppercase().to_string();
            }
            c.to_string()
        })
        .collect()
}

pub fn reminders_to_firebase(reminders: Vec<Reminder>) -> HashMap<String, HashMap<String, Value>> {
    reminders
        .into_iter()
        .map(|r| {
            let mut inner: HashMap<String, Value> = HashMap::new();

            let assignee = match r.assignee {
                Some(a) => Value::String(a.into()),
                None => Value::Null,
            };

            inner.insert("title".into(), Value::String(r.title));
            inner.insert("due".into(), Value::Number(r.due.into()));
            inner.insert("priority".into(), Value::Number(r.priority.into()));
            inner.insert("assignee".into(), assignee);

            (r.id.unwrap_or_default(), inner)
        })
        .collect()
}

/// Reminder model
///
/// When serializing, the id field is skipped if it is None.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Reminder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub due: u64,
    pub priority: u64,
    pub assignee: Option<String>,
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
                title: fix_case(content["title"].as_str().unwrap_or("")),
                due: content["due"].as_u64().unwrap_or(0),
                priority: content["priority"].as_u64().unwrap_or(0),
                assignee: match content.get("assignee") {
                    None => None,
                    Some(value) => match value {
                        Value::String(_) => Some(value.as_str().expect("Not a string").to_string()),
                        _ => None,
                    },
                },
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
        if self.id == other.id {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::reminder::{fix_case, Reminder};
    use std::collections::HashMap;

    /// Test whether the `Reminder::from_json` correctly interprets a Firebase response.
    #[test]
    fn test_from_json() {
        let mut outer: HashMap<String, HashMap<String, serde_json::Value>> = HashMap::new();
        let mut inner: HashMap<String, serde_json::Value> = HashMap::new();

        inner.insert("title".into(), "Hello, world".into());
        inner.insert("due".into(), 1234.into());
        inner.insert("priority".into(), 0.into());
        inner.insert("assignee".into(), serde_json::Value::Null);
        outer.insert("abc".into(), inner);

        let r = vec![Reminder {
            title: "Hello, world".into(),
            due: 1234,
            id: Some("abc".into()),
            priority: 0,
            assignee: None,
        }];

        assert_eq!(r, Reminder::from_json(outer))
    }

    #[test]
    fn test_from_json_invalid_assignee() {
        let mut outer: HashMap<String, HashMap<String, serde_json::Value>> = HashMap::new();
        let mut inner: HashMap<String, serde_json::Value> = HashMap::new();

        inner.insert("title".into(), "Hello, world".into());
        inner.insert("due".into(), 1234.into());
        inner.insert("priority".into(), 0.into());
        inner.insert("assignee".into(), 123.into());
        outer.insert("abc".into(), inner);

        let r = vec![Reminder {
            title: "Hello, world".into(),
            due: 1234,
            id: Some("abc".into()),
            priority: 0,
            assignee: None,
        }];

        assert_eq!(r, Reminder::from_json(outer))
    }

    /// Test that a Reminder serialises properly when no ID is set.
    #[test]
    fn test_serialising_with_no_id() {
        let r = Reminder {
            title: "Hello, world".into(),
            due: 1234,
            id: None,
            priority: 0,
            assignee: Some("Sam".into()),
        };

        let json = serde_json::to_string(&r).unwrap();

        assert_eq!(
            json,
            r#"{"title":"Hello, world","due":1234,"priority":0,"assignee":"Sam"}"#
        )
    }

    /// Test serialising a Reminder with an ID.
    #[test]
    fn test_serialising_with_id() {
        let r = Reminder {
            title: "Hello, world".into(),
            due: 1234,
            id: Some("asdf".into()),
            priority: 0,
            assignee: None,
        };

        let json = serde_json::to_string(&r).unwrap();

        assert_eq!(
            json,
            r#"{"id":"asdf","title":"Hello, world","due":1234,"priority":0,"assignee":null}"#
        )
    }

    /// Test the fix_case function.
    #[test]
    fn test_fix_case() {
        assert_eq!("Hello, world", fix_case("hello, world"));
        assert_eq!("HELLO, WORLD", fix_case("HELLO, WORLD"));
        assert_eq!("Hello, world", fix_case("Hello, world"));
        assert_eq!("Hello, WORLD", fix_case("Hello, WORLD"));
        assert_eq!("!hello, world", fix_case("!hello, world"));
    }
}
