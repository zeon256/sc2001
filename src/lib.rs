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
    use nanorand::{RandomGen, Rng, WyRand};

    use crate::{
        djikstra::*, heap_sort::*, insertion_merge::InsertionMergeSort, insertion_sort::*,
        merge_sort::*, quicksort::*,
    };

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
    fn test_heapsort_random_1000() {
        let mut data = gen_random_array::<1000>();
        let mut data2 = data.clone();
        HeapSort::sort(&mut data);
        data2.sort_unstable();
        assert_eq!(data, data2);
    }
}
