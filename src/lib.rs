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
