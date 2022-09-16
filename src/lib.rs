#![allow(dead_code)]

pub mod djikstra;
pub mod heap_sort;
pub mod insertion_merge;
pub mod insertion_sort;
pub mod merge_sort;
pub mod min_max_search;
pub mod quicksort;

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use std::{fs::File, io::{Read, Write}};

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

    pub fn gen_random_array<const N: usize>() -> Vec<u32> {
        let mut rng = WyRand::new();
        (0..N).map(|_| rng.generate()).collect()
    }

    pub fn assert_sorted<T: Ord>(data: &[T]) {
        for i in 1..data.len() {
            assert!(data[i - 1] <= data[i])
        }
    }

    #[test]
    fn export_all_data_to_csv() {
        let mut all_estimates = String::from("sz,mean,median\n");
        for sz in 3..=512 {
            let mut buf = String::new();
            let file_name = format!("target/criterion/insertion_merge_sort(1mill_s{})/new/estimates.json", sz);
            // let file_name = format!("target/criterion/xd/new/estimates.json");
            println!("{file_name}");
            let mut f = File::open(file_name).unwrap();
            let _ = f.read_to_string(&mut buf).unwrap();
            let estimates = serde_json::from_str::<Estimates>(&buf).unwrap();
            let (mean, median) = (estimates.mean.point_estimate, estimates.median.point_estimate);
            all_estimates.push_str(&format!("{},{},{}\n", sz, mean, median));
            // all_estimates.push((sz, estimates.mean.point_estimate, estimates.median.point_estimate));
        }
        let mut f = File::create("bench.csv").unwrap();
        f.write_all(all_estimates.as_bytes()).unwrap();
    }
}
