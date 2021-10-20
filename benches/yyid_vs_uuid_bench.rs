use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

use uuid::Uuid;
use yyid::Yyid;

fn nil_id_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("nil");
    group.bench_function("uuid", |b| {
        b.iter(|| {
            let id = Uuid::nil();
            let output = format!("{}", id.to_hyphenated_ref());
            black_box(output);
        })
    });
    group.bench_function("yyid", |b| {
        b.iter(|| {
            let id = Yyid::nil();
            let output = format!("{}", id);
            black_box(output);
        })
    });
    group.finish()
}

fn random_id_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("random");
    group.bench_function("uuid", |b| {
        b.iter(|| {
            let id = Uuid::new_v4();
            let output = format!("{}", id.to_hyphenated_ref());
            black_box(output);
        })
    });
    group.bench_function("yyid", |b| {
        b.iter(|| {
            let id = Yyid::new();
            let output = format!("{}", id);
            black_box(output);
        })
    });
    group.finish()
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(60));
    targets = nil_id_bench, random_id_bench
);
criterion_main!(benches);
