use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use mimalloc::MiMalloc;
use nanorand::{Rng, WyRand};
use sc2001::{
    heap_sort::HeapSort, insertion_merge::InsertionMergeSort, merge_sort::MergeSort,
    quicksort::QuickSort,
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

    // let aux_buf = Vec::with_capacity(100000);

    // c.bench_function("merge_sort 1000", |b| {
    //     b.iter_batched(
    //         || rand_array_1th.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("merge_sort 10000", |b| {
    //     b.iter_batched(
    //         || rand_array_10th.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("quick_sort 10000", |b| {
    //     b.iter_batched(
    //         || rand_array_10th.clone(),
    //         |mut data| QuickSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("heap_sort 10000", |b| {
    //     b.iter_batched(
    //         || rand_array_10th.clone(),
    //         |mut data| HeapSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("merge_sort 100000", |b| {
    //     b.iter_batched(
    //         || rand_array_100th.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("insertion_merge_sort 100000", |b| {
    //     b.iter_batched(
    //         || rand_array_1mill.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("heap_sort 100000", |b| {
    //     b.iter_batched(
    //         || rand_array_100th.clone(),
    //         |mut data| HeapSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("quick_sort 1mill", |b| {
    //     b.iter_batched(
    //         || rand_array_1mill.clone(),
    //         |mut data| QuickSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("heap_sort 1mill", |b| {
    //     b.iter_batched(
    //         || rand_array_1mill.clone(),
    //         |mut data| HeapSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("merge_sort 1mill", |b| {
    //     b.iter_batched(
    //         || rand_array_1mill.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    for sz in 3..=512 {
        c.bench_function(&format!("insertion_merge_sort 1mill S = {}", sz), |b| {
            b.iter_batched(
                || rand_array_1mill.clone(),
                |mut data| InsertionMergeSort::sort(&mut data, sz),
                BatchSize::LargeInput,
            )
        });
    }

    // c.bench_function("insertion_merge_sort 10mill S = 65", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| InsertionMergeSort::<65>::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("merge_sort 10mill", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("quicksort 10mill", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| QuickSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("heap_sort 10mill", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| HeapSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("pdqsort_std 10mill", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| data.sort_unstable(),
    //         BatchSize::LargeInput,
    //     )
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
