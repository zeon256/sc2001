/// # Time Complexity
/// - Best Case: O(nlgn)
/// - Worst Case: O(nlgn)
///
/// # Space Complexity
/// - O(n) since we used an auxiliary vector to store the data
pub struct MergeSort;

impl MergeSort {
    pub fn merge_key_cmp<T: Copy + Ord>(l: &mut [T], r: &mut [T], key_cmp: &mut u64) {
        let (sz, mut l_i, mut r_i) = (l.len() + r.len(), 0, 0);

        let mut aux_buf = Vec::with_capacity(sz);

        // loop until the shorter array ends
        // we simply push the element by comparing the first of each
        // we can do this because each l and r are already sorted
        // from the base case
        while l_i < l.len() && r_i < r.len() {
            *key_cmp += 1;

            if l[l_i] < r[r_i] {
                aux_buf.push(l[l_i]);
                l_i += 1;
            } else {
                aux_buf.push(r[r_i]);
                r_i += 1;
            }
        }

        // copy remaining
        for i in l_i..l.len() {
            aux_buf.push(l[i]);
        }

        // copy remaining
        // one of these 2 loops wont run since the condition for the above for
        // loop exits
        for i in r_i..r.len() {
            aux_buf.push(r[i]);
        }

        // replace original slice with new merged data
        let mut aux_i = 0;

        // we set the original buffer to to the new combined aux_buf
        // we need to track the aux_buf index
        for i in 0..l.len() {
            l[i] = aux_buf[aux_i];
            aux_i += 1;
        }

        // same here as above for loop
        for i in 0..r.len() {
            r[i] = aux_buf[aux_i];
            aux_i += 1;
        }
    }
    
    pub fn merge<T: Copy + Ord>(l: &mut [T], r: &mut [T]) {
        let (sz, mut l_i, mut r_i) = (l.len() + r.len(), 0, 0);

        let mut aux_buf = Vec::with_capacity(sz);

        // loop until the shorter array ends
        // we simply push the element by comparing the first of each
        // we can do this because each l and r are already sorted
        // from the base case
        while l_i < l.len() && r_i < r.len() {

            if l[l_i] < r[r_i] {
                aux_buf.push(l[l_i]);
                l_i += 1;
            } else {
                aux_buf.push(r[r_i]);
                r_i += 1;
            }
        }

        // copy remaining
        for i in l_i..l.len() {
            aux_buf.push(l[i]);
        }

        // copy remaining
        // one of these 2 loops wont run since the condition for the above for
        // loop exits
        for i in r_i..r.len() {
            aux_buf.push(r[i]);
        }

        // replace original slice with new merged data
        let mut aux_i = 0;

        // we set the original buffer to to the new combined aux_buf
        // we need to track the aux_buf index
        for i in 0..l.len() {
            l[i] = aux_buf[aux_i];
            aux_i += 1;
        }

        // same here as above for loop
        for i in 0..r.len() {
            r[i] = aux_buf[aux_i];
            aux_i += 1;
        }
    }

    pub fn merge_prealloc<T: Copy + Ord>(l: &mut [T], r: &mut [T], aux_buf: &mut Vec<T>) {
        aux_buf.clear();

        let (mut l_i, mut r_i) = (0, 0);

        // loop until the shorter array ends
        // we simply push the element by comparing the first of each
        // we can do this because each l and r are already sorted
        // from the base case
        while l_i < l.len() && r_i < r.len() {
            if l[l_i] < r[r_i] {
                aux_buf.push(l[l_i]);
                l_i += 1;
            } else {
                aux_buf.push(r[r_i]);
                r_i += 1;
            }
        }

        // copy remaining
        // one of these 2 loops wont run since the condition for the above for
        // loop exits
        for i in l_i..l.len() {
            aux_buf.push(l[i]);
        }

        for i in r_i..r.len() {
            aux_buf.push(r[i]);
        }

        // replace original slice with new merged data
        let mut aux_i = 0;

        // we set the original buffer to to the new combined aux_buf
        // we need to track the aux_buf index
        for i in 0..l.len() {
            l[i] = aux_buf[aux_i];
            aux_i += 1;
        }

        // same here as above for loop
        for i in 0..r.len() {
            r[i] = aux_buf[aux_i];
            aux_i += 1;
        }
    }
}

impl MergeSort {
    pub fn sort<T: Ord + Copy>(buf: &mut [T]) {
        // base case
        if buf.len() == 1 {
            return;
        }

        let (l_buf, r_buf) = buf.split_at_mut(buf.len() / 2);

        Self::sort(l_buf);
        Self::sort(r_buf);
        Self::merge(l_buf, r_buf);
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::{assert_sorted, gen_random_array};

    use super::MergeSort;

    #[test]
    fn test_merge_sort_random() {
        let mut data = gen_random_array::<10000, _>(None);
        MergeSort::sort(&mut data);
        assert_sorted(&data);
    }
}
