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
        // we go through every element
        for i in 0..buf.len() {
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
