pub struct QuickSort;

impl QuickSort {
    pub fn partition<T: Ord + Copy>(buf: &mut [T]) -> usize {
        // we use the middle as the pivot element
        // can actl use other places as pivot but middle makes alot of sense
        let mid = buf.len() / 2;

        // we swap the pivot with the first element in the list
        let pivot = buf[mid];
        buf.swap(0, mid);

        // set to the first item in buf
        // usually in other languages we use the int passed into the function
        let mut last_small = 0;

        // in this case, we are moving all the elements smaller than pivot
        // to the left
        // which also makes the right automatically bigger than the pivot
        for i in 1..buf.len() {
            if buf[i] < pivot {
                // last_small go next first
                last_small += 1;
                buf.swap(last_small, i);
            }
        }

        // we swap back the pivot back to the original position
        buf.swap(0, last_small);

        last_small
    }

    pub fn sort<T: Ord + Copy>(buf: &mut [T]) {
        if !buf.is_empty() {
            let pivot_idx = Self::partition(buf);
            QuickSort::sort(&mut buf[0..pivot_idx]);
            QuickSort::sort(&mut buf[pivot_idx + 1..]);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test::{gen_random_array, assert_sorted};

    use super::QuickSort;

    #[test]
    fn test_quicksort_random_1k() {
        let mut data = gen_random_array::<1000, _>(None);
        QuickSort::sort(&mut data);
        assert_sorted(&data);
    }

    #[test]
    fn test_quicksort_random_10k() {
        let mut data = gen_random_array::<10000, _>(None);
        QuickSort::sort(&mut data);
        assert_sorted(&data);
    }

    #[test]
    fn test_quicksort_random_100k() {
        let mut data = gen_random_array::<100000, _>(None);
        QuickSort::sort(&mut data);
        assert_sorted(&data);
    }

    #[test]
    fn test_quicksort_random_1mill() {
        let mut data = gen_random_array::<1000000, _>(None);
        QuickSort::sort(&mut data);
        assert_sorted(&data);
    }

}
