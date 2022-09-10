use crate::{insertion_sort::InsertionSort, merge_sort::MergeSort, Sort};

/// # Type parameter
/// 
/// * `N` - Threshold to swap to insertion sort
pub struct InsertionMergeSort<const N: usize>;

impl<const N: usize> InsertionMergeSort<N> {
    pub fn sort<T: Ord + Copy>(buf: &mut [T], aux_buf: &mut Vec<T>) {
        let sz = buf.len();
        if sz <= N {
            InsertionSort::sort(buf);
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        Self::sort(l_buf, aux_buf);
        Self::sort(r_buf, aux_buf);
        MergeSort::merge_prealloc(l_buf, r_buf, aux_buf);
    }
}
