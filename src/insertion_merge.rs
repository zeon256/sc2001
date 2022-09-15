use crate::{insertion_sort::InsertionSort, merge_sort::MergeSort};

/// # Type parameter
///
/// * `S` - Threshold to swap to insertion sort
pub struct InsertionMergeSort<const S: usize>;

impl<const S: usize> InsertionMergeSort<S> {
    pub fn sort<T: Ord + Copy>(buf: &mut [T]) {
        let sz = buf.len();

        if sz <= S {
            #[cfg(debug_assertions)]
            println!("Array sz <= S");

            InsertionSort::sort(buf);
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        Self::sort(l_buf);
        Self::sort(r_buf);
        MergeSort::merge(l_buf, r_buf);
    }
}

#[cfg(test)]
mod test {
    use crate::{insertion_merge::InsertionMergeSort, test::{gen_random_array, assert_sorted}};

    #[test]
    fn test_insertion_merge_sort_random() {
        let mut data = gen_random_array::<10000>();
        InsertionMergeSort::<15>::sort(&mut data);
        assert_sorted(&data);
    }
}
