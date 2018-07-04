extern crate sequoia;

use std::io;

use sequoia::json_encoder::JsonEncoder;
use sequoia::{Entry, Level, Logger};

fn main() {
    let mut l = Logger::new(io::stdout());
    // Log something at the info level.
    l.log(
        Entry::new(Some(Level::Info), JsonEncoder)
            .str("key", "value")
            .msg("testing"),
    );

    // Entries without a level will always be logged.
    l.log(
        Entry::new(None, JsonEncoder)
            .str("time", "now")
            .msg("another line but without a level"),
    );

    // This debug log will not be logged since it is lower than info level.
    l.log(Entry::new(Some(Level::Debug), JsonEncoder).msg("will not be seen"));

    l.log(Entry::new(Some(Level::Warn), JsonEncoder).msg("a warning"));
}
