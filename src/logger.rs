use std::io::{BufWriter, Write};

use field::Entry;

/// `Level` represents the logging level for a specific line.
pub enum Level {
    Debug,
    Error,
    Fatal,
    Info,
    Warn,
}

/// `Logger` can serialize logs to anyting that implements `Write`.
pub struct Logger<W: Write> {
    location: BufWriter<W>,
}

impl<W: Write> Logger<W> {
    /// Create a new `Logger` for the given location.
    ///
    /// # Example
    /// ```rust
    /// use std::io;
    ///
    /// use sequoia::Logger;
    ///
    /// let l = Logger::new(io::stdout());
    /// ```
    pub fn new(location: W) -> Logger<W> {
        Logger {
            location: BufWriter::new(location),
        }
    }

    // FIXME: Make a macro and generalize this for all levels.
    pub fn info(&mut self, msg: &str, fields: &mut Entry) {
        fields.set_level(Level::Info);
        fields.add_field("msg", msg);
        let line = fields.finish();
        self.location.write_all(line.as_bytes()).unwrap();
    }
}
