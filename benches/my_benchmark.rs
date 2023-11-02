use base94::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let random_bytes_1000 = (0..1000).map(|_| rand::random::<u8>()).collect::<Vec<u8>>();
    let random_bytes_10000 = (0..10000)
        .map(|_| rand::random::<u8>())
        .collect::<Vec<u8>>();

    c.bench_function("encode_1000", |b| b.iter(|| encode(&random_bytes_1000, 94)));
    c.bench_function("encode_10000", |b| {
        b.iter(|| encode(&random_bytes_10000, 94))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
