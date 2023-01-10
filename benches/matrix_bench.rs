use criterion::{criterion_group, criterion_main, Criterion};

use rust_geometry::Matrix;

fn bench_default_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("Default Construction");
    group.bench_function("Array Default Construction", |b| {
        b.iter(|| [[i32::default(); 3]; 2])
    });
    group.bench_function("Matrix Default Construction", |b| {
        b.iter(|| Matrix::<2, 3, i32>::default())
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_default_construction
);
criterion_main!(benches);
