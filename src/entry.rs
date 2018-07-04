use std::sync::Arc;

use parking_lot::Mutex;
use pool::Pool;

use encoder::Encoder;
use logger::Level;

const DEFAULT_POOL_CAPACITY: usize = 1024;
const DEFAULT_BUFFER_SIZE: usize = 2048;

// TODO: How can we make this smarter so it doesn't put the buffer back in the pool when it is
// dropped?
lazy_static! {
    static ref BUFFER_POOL: Arc<Mutex<Pool<Vec<u8>>>> = {
        let p = Arc::new(Mutex::new(Pool::with_capacity(
            DEFAULT_POOL_CAPACITY,
            0,
            || Vec::with_capacity(DEFAULT_BUFFER_SIZE),
        )));
        p
    };
}

/// An `Entry` represents an individual log line made up of `Field`s.
pub struct Entry<E: Encoder> {
    encoder: E,
    buffer: Vec<u8>,
}

impl<E: Encoder> Entry<E> {
    /// Return a blank `Entry`.
    pub fn new(level: Option<Level>, encoder: E) -> Entry<E> {
        let mut buffer = vec![];
        encoder.append_start(&mut buffer);

        // TODO: Make a lookup map for the levels or just to `ToString`?
        if let Some(_) = level {
            encoder.append_string(&mut buffer, "level", "info");
        }

        Entry {
            encoder: encoder,
            buffer: buffer,
        }
    }

    /// Add a string value to the `Entry`.
    pub fn str(mut self, key: &str, value: &str) -> Entry<E> {
        self.encoder.append_string(&mut self.buffer, key, value);
        self
    }

    /// Add the top-level message for this `Entry` which completes the log message. This message
    /// will now be ready to be added to the logger.
    pub fn msg(mut self, msg: &str) -> Entry<E> {
        self.encoder.append_string(&mut self.buffer, "msg", msg);
        self.encoder.append_end(&mut self.buffer);
        self
    }
}
