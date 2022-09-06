use crate::{insertion_sort::InsertionSort, merge_sort::MergeSort, Sort};

pub struct InsertionMergeSort<const N: usize>;

impl<const N: usize> Sort for InsertionMergeSort<N> {
    fn sort<T: Copy + Ord>(buf: &mut [T]) {
        // base case
        if buf.len() == 1 {
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        if l_buf.len() < N {
            InsertionSort::sort(l_buf);
        } else {
            MergeSort::sort(r_buf);
        }

        if r_buf.len() < N {
            InsertionSort::sort(l_buf);
        } else {
            MergeSort::sort(r_buf);
        }

        MergeSort::merge(l_buf, r_buf);
    }
}
