#[macro_use]
extern crate criterion;

extern crate sequoia;

use criterion::Criterion;
use sequoia::json_encoder::JsonEncoder;
use sequoia::{Entry, Level, Logger};

fn criterion_benchmark(c: &mut Criterion) {
    let mut l = Logger::new(::std::io::sink());
    c.bench_function("simple log", move |b| {
        b.iter(|| {
            l.log(
                Entry::new(Some(Level::Info), JsonEncoder)
                    .str("key", "value")
                    .integer("integer", 7)
                    .msg("top level message"),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
