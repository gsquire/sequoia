#[macro_use]
extern crate serde_json;

mod field;
mod logger;

pub use field::Entry;
pub use logger::Logger;
