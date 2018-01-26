extern crate sequoia;

use std::io;

use sequoia::{Entry, Logger};

fn main() {
    let mut l = Logger::new(io::stdout());
    let mut entry = Entry::new();
    entry.add_field("count", 7);

    l.info("test", &mut entry);
}
