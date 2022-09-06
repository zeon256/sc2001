use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use mimalloc::MiMalloc;
use nanorand::{Rng, WyRand};
use sc2001::{
    insertion_merge::{InsertionMergeSort, InsertionMergeSort2}, insertion_sort::InsertionSort, merge_sort::MergeSort, Sort,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn gen_random_array<const N: usize>() -> Vec<u32> {
    let mut rng = WyRand::new_seed(420);
    (0..N).map(|_| rng.generate()).collect::<Vec<_>>()
}

fn criterion_benchmark(c: &mut Criterion) {
    let rand_array_1th = gen_random_array::<1000>();
    let rand_array_10th = gen_random_array::<1_0000>();
    let rand_array_100th = gen_random_array::<1_00000>();
    let rand_array_1mill = gen_random_array::<1_000_000>();
    let rand_array_10mill = gen_random_array::<10_000_000>();

    c.bench_function("merge_sort 1000", |b| {
        b.iter_batched(
            || rand_array_1th.clone(),
            |mut data| MergeSort::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("merge_sort 10000", |b| {
        b.iter_batched(
            || rand_array_10th.clone(),
            |mut data| MergeSort::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("merge_sort 100000", |b| {
        b.iter_batched(
            || rand_array_100th.clone(),
            |mut data| MergeSort::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("merge_sort 1mill", |b| {
        b.iter_batched(
            || rand_array_1mill.clone(),
            |mut data| MergeSort::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("merge_sort 10mill", |b| {
        b.iter_batched(
            || rand_array_10mill.clone(),
            |mut data| MergeSort::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("insertion_merge_sort 10mill", |b| {
        b.iter_batched(
            || rand_array_10mill.clone(),
            |mut data| InsertionMergeSort::<20>::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("insertion_merge_sort2 10mill", |b| {
        b.iter_batched(
            || rand_array_10mill.clone(),
            |mut data| InsertionMergeSort2::<20>::sort(&mut data),
            BatchSize::LargeInput,
        )
    });

    c.bench_function("std_unstable 10mil", |b| {
        b.iter_batched(
            || rand_array_10mill.clone(),
            |mut data| data.sort_unstable(),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
