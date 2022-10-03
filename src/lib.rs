#![allow(dead_code)]

pub mod djikstra;
pub mod heap_sort;
pub mod insertion_merge;
pub mod insertion_sort;
pub mod merge_sort;
pub mod min_max_search;
pub mod quicksort;
pub mod graph;
pub mod union_find;

#[cfg(test)]
#[allow(unused_imports)]
pub mod test {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    use nanorand::{RandomGen, Rng, WyRand};
    use serde::{Deserialize, Serialize};
    use serde_json;

    use crate::{
        djikstra::*, heap_sort::*, insertion_merge::InsertionMergeSort, insertion_sort::*,
        merge_sort::*, quicksort::*,
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct Estimates {
        mean: EstimateData,
        median: EstimateData,
        median_abs_dev: EstimateData,
        std_dev: EstimateData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct EstimateData {
        confidence_interval: ConfidenceInterval,
        point_estimate: f64,
        standard_error: f64,
    }
    #[derive(Serialize, Deserialize, Debug)]
    struct ConfidenceInterval {
        confidence_level: f64,
        lower_bound: f64,
        upper_bound: f64,
    }

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

    #[test]
    fn export_all_data_to_csv() {
        let array_sz = ["1k", "10k", "100k", "500k", "1mill"];

        for name in array_sz {
            let mut all_estimates = String::from("sz,mean,median\n");
            for sz in 3..=128 {
                let mut buf = String::new();
                let file_name = format!(
                    "target/criterion/insertion_merge_sort({name}_s{sz})/new/estimates.json"
                );

                println!("{file_name}");

                let mut f = File::open(file_name).unwrap();
                let _ = f.read_to_string(&mut buf).unwrap();

                let estimates = serde_json::from_str::<Estimates>(&buf).unwrap();
                let (mean, median) = (
                    estimates.mean.point_estimate / 1000000.0,
                    estimates.median.point_estimate / 1000000.0,
                );

                all_estimates.push_str(&format!("{sz},{mean},{median}\n",));
            }

            let mut f = File::create(format!("bench_{name}.csv")).unwrap();
            f.write_all(all_estimates.as_bytes()).unwrap();
        }
    }
}
