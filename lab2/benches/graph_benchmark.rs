use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use mimalloc::MiMalloc;
use nanorand::{Rng, WyRand};

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

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
