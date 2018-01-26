use serde_json::{self, Value};

use logger::Level;

// TODO: A trait that can wrap anything that a Value can serialize.

/// An `Entry` represents an individual log line made up of `Field`s.
pub struct Entry {
    fields: Value,
}

impl Entry {
    /// Return a blank `Entry`.
    pub fn new() -> Entry {
        Entry {
            fields: json!({}),
        }
    }

    /// Set the level for this log entry.
    // TODO: Implement a ToString method for the Level enum.
    pub fn set_level(&mut self, _level: Level) {
        self.add_field("level", "info");
    }

    /// Add a new field to the `Entry`.
    pub fn add_field<F: Into<Value>>(&mut self, key: &str, value: F) {
        self.fields[key] = value.into();
    }

    /// Serialize the fields and return the string.
    pub fn finish(&self) -> String {
        serde_json::to_string(&self.fields).unwrap()
    }
}
