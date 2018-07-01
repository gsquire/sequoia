use std::io::{BufWriter, Write};

use pool::Pool;

use field::Entry;

const DEFAULT_POOL_CAPACITY: usize = 1024;
const DEFAULT_BUFFER_SIZE: usize = 2048;

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
    output: BufWriter<W>,
    buffer_pool: Pool<Vec<u8>>,
}

impl<W: Write> Logger<W> {
    /// Create a new `Logger` for the given output.
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
        let buffer_pool = Pool::with_capacity(DEFAULT_POOL_CAPACITY, 0, || {
            Vec::with_capacity(DEFAULT_BUFFER_SIZE)
        });
        Logger {
            output: BufWriter::new(output),
            buffer_pool: buffer_pool,
        }
    }
}
