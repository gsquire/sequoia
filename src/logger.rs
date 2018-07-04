use std::io::{BufWriter, Write};

use encoder::Encoder;
use entry::Entry;

/// `Level` represents the logging level for a specific line.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Level {
    /// Debug is the lowest level (0).
    Debug = 0,
    /// Info is second lowest level (1).
    Info,
    /// Warn is the third lowest level (2).
    Warn,
    /// Error is the second second highest level (3).
    Error,
    /// Fatal is the highest level (4).
    Fatal,
}

/// `Logger` can serialize logs to anyting that implements `Write`.
pub struct Logger<W: Write> {
    output: BufWriter<W>,
    level: Level,
}

impl<W: Write> Logger<W> {
    /// Create a new `Logger` for the given output. The default level is Info.
    ///
    /// # Example
    /// ```rust
    /// use std::io;
    ///
    /// use sequoia::Logger;
    ///
    /// let l = Logger::new(io::stdout());
    /// ```
    pub fn new(output: W) -> Logger<W> {
        Logger {
            output: BufWriter::new(output),
            level: Level::Info,
        }
    }

    /// Set the level of the logger.
    ///
    /// # Example
    /// ```ignore
    /// let mut l = Logger::new(io::stdout());
    /// l.set_level(Level::Debug);
    /// ```
    pub fn set_level(&mut self, l: Level) {
        self.level = l;
    }

    /// Write an `Entry` to the writer.
    pub fn log<E: Encoder>(&mut self, mut entry: Entry<E>) {
        // We default to Fatal since that will always be printed.
        let entry_level = entry.level().unwrap_or(Level::Fatal);

        if entry_level >= self.level {
            if let Err(e) = self.output.write_all(entry.finish()) {
                eprintln!("error writing log line: {}", e);
            }
        }
    }
}
