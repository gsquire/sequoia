use std::io::{BufWriter, Write};

use pool::Pool;

use encoder::Encoder;
use field::Entry;

const DEFAULT_POOL_CAPACITY: usize = 1024;
const DEFAULT_BUFFER_SIZE: usize = 2048;

/// `Level` represents the logging level for a specific line.
pub enum Level {
    /// Debug is the lowest level (0).
    Debug,
    /// Error is the second second highest level (3).
    Error,
    /// Fatal is the highest level (4).
    Fatal,
    /// Info is second lowest level (1).
    Info,
    /// Warn is the third lowest level (2).
    Warn,
}

/// `Logger` can serialize logs to anyting that implements `Write`.
pub struct Logger<E: Encoder, W: Write> {
    encoder: Option<E>,
    output: BufWriter<W>,
    buffer_pool: Pool<Vec<u8>>,
    level: Level,
}

impl<E: Encoder, W: Write> Logger<E, W> {
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
    pub fn new(output: W) -> Logger<E, W> {
        let buffer_pool = Pool::with_capacity(DEFAULT_POOL_CAPACITY, 0, || {
            Vec::with_capacity(DEFAULT_BUFFER_SIZE)
        });
        Logger {
            encoder: None,
            output: BufWriter::new(output),
            buffer_pool: buffer_pool,
            level: Level::Info,
        }
    }

    /// Set the level of the logger.
    ///
    /// # Example
    /// ```rust, no_run
    /// let mut l = Logger::new(io::stdout());
    /// l.set_level(Level::Debug);
    /// ```
    pub fn set_level(&mut self, l: Level) {
        self.level = l;
    }

    /// Set the `Encoder` that will encode each log line.
    pub fn set_encoder(&mut self, encoder: E) {
        self.encoder = Some(encoder);
    }
}
