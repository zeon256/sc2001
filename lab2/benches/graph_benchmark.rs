use criterion::{criterion_group, criterion_main, Criterion};
use mimalloc::MiMalloc;
use nanorand::{Rng, WyRand};
use lab2::graph::{MatrixGraph, ListGraph};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn criterion_benchmark(c: &mut Criterion) {

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
