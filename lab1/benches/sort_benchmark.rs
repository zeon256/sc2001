use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use mimalloc::MiMalloc;
use nanorand::{Rng, WyRand};
use sc2001::{
    heap_sort::HeapSort, insertion_merge::InsertionMergeSort, merge_sort::MergeSort,
    quicksort::QuickSort,
};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn gen_random_array<const N: usize, T>(seed: T) -> Vec<u32>
where
    T: Into<Option<u64>>,
{
    // let mut rng = WyRand::new();
    let mut rng = match seed.into() {
        Some(seed) => WyRand::new_seed(seed),
        None => WyRand::new(),
    };

    (0..N).map(|_| rng.generate()).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let rand_array_1k = gen_random_array::<1000, _>(420);
    let rand_array_10k = gen_random_array::<1_0000, _>(421);
    let rand_array_100k = gen_random_array::<1_00000, _>(422);
    let rand_array_500k = gen_random_array::<5_00000, _>(422);
    let rand_array_1mill = gen_random_array::<1_000_000, _>(423);
    let rand_array_10mill = gen_random_array::<10_000_000, _>(424);

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

    // for sz in 3..=512 {
    //     c.bench_function(&format!("insertion_merge_sort(1mill_s{})", sz), |b| {
    //         b.iter_batched(
    //             || rand_array_1mill.clone(),
    //             |mut data| InsertionMergeSort::sort(&mut data, sz),
    //             BatchSize::LargeInput,
    //         )
    //     });
    // }

    #[cfg(not(feature = "key_cmp"))]
    for sz in 3..=128 {
        c.bench_function(&format!("insertion_merge_sort(1k_s{})", sz), |b| {
            b.iter_batched(
                || rand_array_1k.clone(),
                |mut data| InsertionMergeSort::sort(&mut data, sz),
                BatchSize::SmallInput,
            )
        });
    }

    #[cfg(not(feature = "key_cmp"))]
    for sz in 3..=128 {
        c.bench_function(&format!("insertion_merge_sort(10k_s{})", sz), |b| {
            b.iter_batched(
                || rand_array_10k.clone(),
                |mut data| InsertionMergeSort::sort(&mut data, sz),
                BatchSize::SmallInput,
            )
        });
    }
    
    #[cfg(not(feature = "key_cmp"))]
    for sz in 3..=128 {
        c.bench_function(&format!("insertion_merge_sort(100k_s{})", sz), |b| {
            b.iter_batched(
                || rand_array_100k.clone(),
                |mut data| InsertionMergeSort::sort(&mut data, sz),
                BatchSize::SmallInput,
            )
        });
    }

    #[cfg(not(feature = "key_cmp"))]
    for sz in 3..=128 {
        c.bench_function(&format!("insertion_merge_sort(500k_s{})", sz), |b| {
            b.iter_batched(
                || rand_array_500k.clone(),
                |mut data| InsertionMergeSort::sort(&mut data, sz),
                BatchSize::SmallInput,
            )
        });
    }
    
    #[cfg(not(feature = "key_cmp"))]
    for sz in 3..=128 {
        c.bench_function(&format!("insertion_merge_sort(1mill_s{})", sz), |b| {
            b.iter_batched(
                || rand_array_1mill.clone(),
                |mut data| InsertionMergeSort::sort(&mut data, sz),
                BatchSize::LargeInput,
            )
        });
    }

    // c.bench_function("merge_sort(10mill_s20)", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| MergeSort::sort(&mut data),
    //         BatchSize::LargeInput,
    //     )
    // });

    // #[cfg(not(feature = "key_cmp"))]
    // c.bench_function("insertion_merge_sort(10mill_s20)", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| InsertionMergeSort::sort(&mut data, 20),
    //         BatchSize::LargeInput,
    //     )
    // });

    // c.bench_function("insertion_merge_sort(std_lib)", |b| {
    //     b.iter_batched(
    //         || rand_array_10mill.clone(),
    //         |mut data| data.sort(),
    //         BatchSize::LargeInput,
    //     )
    // });


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
