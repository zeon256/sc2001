use crate::Sort;

/// # Time Complexity
/// - Best Case: O(n)
/// - Worst Case: O(n^2)
///
/// # Space Complexity
/// - O(1) since no auxiliary data structures were used
pub struct InsertionSort;

impl Sort for InsertionSort {
    fn sort<T: Ord>(buf: &mut [T]) {
        // we go through every element except 1 because 
        // the first element is assumed to be sorted
        for i in 1..buf.len() {
            // for 1 subsequent element, we keep swapping when curr is smaller than previous element
            // keep swapping until we reach an element that is already smaller
            for j in (1..=i).rev() {
                if buf[j] < buf[j - 1] {
                    // swap
                    buf.swap(j, j - 1)
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{insertion_sort::InsertionSort, test::gen_random_array, Sort};


    #[test]
    fn test_insertion_sort_random() {
        let mut data = gen_random_array::<10000>();
        let mut data2 = data.clone();
        InsertionSort::sort(&mut data);
        data2.sort_unstable();
        assert_eq!(data, data2);
    }
}
