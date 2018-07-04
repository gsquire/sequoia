extern crate sequoia;

use std::io;

use sequoia::json_encoder::JsonEncoder;
use sequoia::{Entry, Level, Logger};

fn main() {
    let mut l = Logger::new(io::stdout());
    l.info(
        Entry::new(Some(Level::Info), JsonEncoder)
            .str("key", "value")
            .msg("testing"),
    );
}
