use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mimalloc::MiMalloc;
use nanorand::{WyRand, Rng};
use sc2001::{insertion_sort::InsertionSort, Sort};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn gen_random_array() -> Vec<i32> {
    let mut rng = WyRand::new_seed(420);
    let mut buf = vec![0; 1_000_000];
    for i in 0..1_000_000 {
        buf[i] = rng.generate();
    }
    println!("Generation complete!");
    buf
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rand_array_0 = gen_random_array();
    // let mut rand_array_1 = rand_array_0.clone();
    // let mut random_array = vec![1, 7, 3, 2, 1, 5];
    c.bench_function("insertion 1_000_000", |b| {
        b.iter(|| InsertionSort::sort(black_box(&mut rand_array_0)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
