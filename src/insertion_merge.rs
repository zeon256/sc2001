use crate::{insertion_sort::InsertionSort, merge_sort::MergeSort, Sort};

pub struct InsertionMergeSort<const N: usize>;

impl<const N: usize> Sort for InsertionMergeSort<N> {
    fn sort<T: Copy + Ord>(buf: &mut [T]) {
        let sz = buf.len();
        if sz <= N {
            InsertionSort::sort(buf);
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        Self::sort(l_buf);
        Self::sort(r_buf);
        MergeSort::merge(l_buf, r_buf);
    }
}
