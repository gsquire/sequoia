/// Encoder represents how logs will be written to their underlying store.
pub trait Encoder {
    /// Append the starting delimiter to the buffer if there is one.
    fn append_start(&self, dst: &mut Vec<u8>);

    /// Append a new key to the log.
    fn append_key(&self, dst: &mut Vec<u8>, key: &str);

    /// Append a string to the buffer.
    fn append_string(&self, dst: &mut Vec<u8>, value: &str);

    /// Append the closing delimiter to the buffer if there is one.
    fn append_end(&self, dst: &mut Vec<u8>);

    /// Append a line break to the log.
    fn append_line_break(&self, dst: &mut Vec<u8>);
}
