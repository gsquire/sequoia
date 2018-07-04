use dtoa::Floating;
use itoa::Integer;
use parking_lot::Mutex;
use pool::{Checkout, Pool};

use encoder::Encoder;
use logger::{self, Level};

const DEFAULT_POOL_CAPACITY: usize = 1024;
const DEFAULT_BUFFER_SIZE: usize = 2048;
const MSG_KEY: &str = "msg";
const LEVEL_KEY: &str = "level";

lazy_static! {
    static ref BUFFER_POOL: Mutex<Pool<Vec<u8>>> = {
        let p = Mutex::new(Pool::with_capacity(DEFAULT_POOL_CAPACITY, 0, || {
            Vec::with_capacity(DEFAULT_BUFFER_SIZE)
        }));
        p
    };
}

fn get_buffer() -> Checkout<Vec<u8>> {
    let mut p = BUFFER_POOL.lock();
    p.checkout().unwrap()
}

/// An `Entry` represents an individual log line made up of `Field`s.
pub struct Entry<E: Encoder> {
    encoder: E,
    level: Option<Level>,
    buffer: Checkout<Vec<u8>>,
}

impl<E: Encoder> Entry<E> {
    /// Return a blank `Entry`.
    pub fn new(level: Option<Level>, encoder: E) -> Entry<E> {
        let mut buffer = get_buffer();
        encoder.append_start(&mut buffer);

        if let Some(ref l) = level {
            let level_value = logger::level_string(l.clone());
            encoder.append_key(&mut buffer, LEVEL_KEY);
            encoder.append_string(&mut buffer, level_value);
        }

        Entry {
            encoder: encoder,
            level: level,
            buffer: buffer,
        }
    }

    /// Get the log level associated with this entry.
    #[inline]
    pub fn level(&self) -> Option<Level> {
        if let Some(ref l) = self.level {
            return Some(l.clone());
        }
        None
    }

    /// Add a string value to the `Entry`.
    pub fn str(mut self, key: &str, value: &str) -> Entry<E> {
        self.encoder.append_key(&mut self.buffer, key);
        self.encoder.append_string(&mut self.buffer, value);
        self
    }

    /// Add an integer type value.
    pub fn integer<V: Integer>(mut self, key: &str, value: V) -> Entry<E> {
        self.encoder.append_key(&mut self.buffer, key);
        self.encoder.append_integer(&mut self.buffer, value);
        self
    }

    /// Add a float type value.
    pub fn float<V: Floating>(mut self, key: &str, value: V) -> Entry<E> {
        self.encoder.append_key(&mut self.buffer, key);
        self.encoder.append_float(&mut self.buffer, value);
        self
    }

    /// Add the top-level message for this `Entry`.
    pub fn msg(mut self, msg: &str) -> Entry<E> {
        self.encoder.append_key(&mut self.buffer, MSG_KEY);
        self.encoder.append_string(&mut self.buffer, msg);
        self
    }

    /// Finish the log message making it ready for the underlying writer.
    pub fn finish(&mut self) -> &[u8] {
        self.encoder.append_end(&mut self.buffer);
        self.encoder.append_line_break(&mut self.buffer);
        &self.buffer
    }
}
