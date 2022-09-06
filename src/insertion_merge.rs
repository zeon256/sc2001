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

pub struct InsertionMergeSort2<const N: usize>;

impl<const N: usize> Sort for InsertionMergeSort2<N> {
    fn sort<T: Copy + Ord>(buf: &mut [T]) {
        // if sz == N {
        //     InsertionSort::sort(buf);
        //     return;
        // }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        if l_buf.len() <= N/2 {
            InsertionSort::sort(l_buf);
        } else {
            Self::sort(l_buf);
        }

        if r_buf.len() <= N/2 {
            InsertionSort::sort(r_buf);
        } else {
            Self::sort(r_buf);
        }

        MergeSort::merge(l_buf, r_buf);
    }
}
