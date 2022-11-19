use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

use uuid::Uuid;
use yyid::Yyid;

fn nil_id_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("nil");
    group.bench_function("uuid", |b| {
        b.iter(|| {
            black_box(Uuid::nil().hyphenated().to_string());
        })
    });
    group.bench_function("yyid", |b| {
        b.iter(|| {
            black_box(Yyid::nil().hyphenated().to_string());
        })
    });
    group.finish()
}

fn random_id_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    group.bench_function("uuid", |b| {
        b.iter(|| {
            black_box(Uuid::new_v4().hyphenated().to_string());
        })
    });
    group.bench_function("yyid", |b| {
        b.iter(|| {
            black_box(Yyid::new().hyphenated().to_string());
        })
    });
    group.finish()
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = nil_id_bench, random_id_bench
);
criterion_main!(benches);
