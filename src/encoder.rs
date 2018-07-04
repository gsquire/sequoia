/// Encoder represents how logs will be written to their underlying store.
pub trait Encoder {
    /// Append the starting delimiter to the buffer if there is one.
    fn append_start(&self, dst: &mut Vec<u8>);

    /// Append a string to the buffer.
    fn append_string(&self, dst: &mut Vec<u8>, key: &str, value: &str);

    /// Append the closing delimiter to the buffer if there is one.
    fn append_end(&self, dst: &mut Vec<u8>);
}
