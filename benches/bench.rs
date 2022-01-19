use criterion::{criterion_group, criterion_main, Criterion};
use flake::IdGenerator;
use std::time::UNIX_EPOCH;

fn criterion_benchmark(c: &mut Criterion) {
    let mut generator = IdGenerator::new(UNIX_EPOCH, 0, 0).unwrap();

    c.bench_function("id", |b| b.iter(|| generator.id()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
