#![allow(dead_code)]

pub mod djikstra;
pub mod graph;
pub mod heap_sort;
pub mod insertion_merge;
pub mod insertion_sort;
pub mod merge_sort;
pub mod min_max_search;
pub mod quicksort;
pub mod union_find;
pub mod lcs;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Estimates {
    pub mean: EstimateData,
    pub median: EstimateData,
    pub median_abs_dev: EstimateData,
    pub std_dev: EstimateData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimateData {
    pub confidence_interval: ConfidenceInterval,
    pub point_estimate: f64,
    pub standard_error: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfidenceInterval {
    pub confidence_level: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

pub mod test_utils {
    use nanorand::{Rng, WyRand};

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

    pub fn assert_sorted<T: Ord>(data: &[T]) {
        for i in 1..data.len() {
            assert!(data[i - 1] <= data[i])
        }
    }
}
