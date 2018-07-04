use std::io::Write;

use encoder::Encoder;

/// A type that can encode log lines as JSON.
pub struct JsonEncoder;

impl Encoder for JsonEncoder {
    fn append_start(&self, dst: &mut Vec<u8>) {
        dst.write(b"{").unwrap();
    }

    fn append_key(&self, dst: &mut Vec<u8>, key: &str) {
        let l = dst.len();
        if l > 1 && dst[l - 1] != b'{' {
            dst.write(b",").unwrap();
        }
        self.append_string(dst, key);
        dst.write(b":").unwrap();
    }

    // TODO: This may need some special encoding.
    fn append_string(&self, dst: &mut Vec<u8>, value: &str) {
        dst.write(b"\"").unwrap();
        dst.write(value.as_bytes()).unwrap();
        dst.write(b"\"").unwrap();
    }

    fn append_end(&self, dst: &mut Vec<u8>) {
        dst.write(b"}").unwrap();
    }

    fn append_line_break(&self, dst: &mut Vec<u8>) {
        dst.write(b"\n").unwrap();
    }
}
