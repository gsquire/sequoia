use logger::Level;

/// An `Entry` represents an individual log line made up of `Field`s.
pub struct Entry {
    level: Option<Level>,
}

impl Entry {
    /// Return a blank `Entry`.
    pub fn new() -> Entry {
        Entry { level: None }
    }
}
