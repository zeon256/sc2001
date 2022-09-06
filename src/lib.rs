#![allow(dead_code)]
pub mod djikstra;
pub mod heap_sort;
pub mod insertion_merge;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quicksort;

pub trait Sort {
    fn sort<T: Copy + Ord>(buf: &mut [T]);
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use nanorand::{WyRand, Rng};

    use crate::{
        djikstra::*, heap_sort::*, insertion_merge::InsertionMergeSort, insertion_sort::*,
        merge_sort::*, quicksort::*, Sort,
    };

    const ARRAY_0: ([i32; 6], [i32; 6]) = ([1, 7, 3, 2, 1, 5], [1, 1, 2, 3, 5, 7]);
    const ARRAY_1: ([i32; 1], [i32; 1]) = ([1], [1]);
    const ARRAY_2: ([i32; 12], [i32; 12]) = (
        [1, 7, 3, 2, 1, 5, 1, 7, 3, 2, 1, 5],
        [1, 1, 1, 1, 2, 2, 3, 3, 5, 5, 7, 7],
    );

    fn gen_random_array<const N: usize>() -> Vec<u32> {
        let mut rng = WyRand::new_seed(420);
        (0..N).map(|_| rng.generate()).collect::<Vec<_>>()
    }

    #[test]
    fn test_insertion_sort() {
        let (mut data, exp) = ARRAY_0.clone();
        InsertionSort::sort(&mut data);
        assert_eq!(data, exp);

        let (mut data, exp) = ARRAY_1.clone();
        InsertionSort::sort(&mut data);
        assert_eq!(data, exp);
    }

    #[test]
    fn test_merge_sort() {
        let (mut data, exp) = ARRAY_0.clone();
        MergeSort::sort(&mut data);
        assert_eq!(data, exp);

        let (mut data, exp) = ARRAY_1.clone();
        MergeSort::sort(&mut data);
        assert_eq!(data, exp);
    }

    #[test]
    fn test_merge_sort_random() {
        let mut data = gen_random_array::<1000>();
        let mut data2 = data.clone();
        MergeSort::sort(&mut data);
        data2.sort_unstable();
        assert_eq!(data, data2);
    }

    #[test]
    fn test_insertion_merge_sort() {
        let (mut data, exp) = ARRAY_0.clone();
        InsertionMergeSort::<10>::sort(&mut data);
        assert_eq!(data, exp);

        let (mut data, exp) = ARRAY_1.clone();
        InsertionMergeSort::<10>::sort(&mut data);
        assert_eq!(data, exp);

        let (mut data, exp) = ARRAY_2.clone();
        InsertionMergeSort::<10>::sort(&mut data);
        assert_eq!(data, exp);
    }
}
