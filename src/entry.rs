use encoder::Encoder;
use logger::Level;

/// An `Entry` represents an individual log line made up of `Field`s.
pub struct Entry<'a, E: Encoder> {
    level: Option<Level>,
    encoder: E,
    buffer: &'a mut Vec<u8>,
}

impl<'a, E: Encoder> Entry<'a, E> {
    /// Return a blank `Entry`.
    pub fn new(level: Level, encoder: E, buffer: &'a mut Vec<u8>) -> Entry<'a, E> {
        Entry {
            level: None,
            encoder: encoder,
            buffer: buffer,
        }
    }
}
