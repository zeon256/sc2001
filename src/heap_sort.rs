pub struct HeapSort;

impl HeapSort {
    pub fn sort<T: Ord>(buf: &mut [T]) {
        if buf.len() <= 1 {
            return;
        }
        // we build the heap first
        Self::build_max_heap(buf);

        // after building max heap
        // we can swap the largest to the last index
        for last in (1..buf.len()).rev() {
            buf.swap(0, last);
            // we reduce the size of the heap
            Self::heapify(&mut buf[..last], 0);
        }
    }

    /// Convert `buf` into a max heap.
    pub fn build_max_heap<T: Ord>(buf: &mut [T]) {
        let last_parent = (buf.len() - 2) / 2;
        for i in (0..=last_parent).rev() {
            Self::heapify(buf, i);
        }
    }

    /// Fixes 1 violation at a time then recursively fix affected subtree
    pub fn heapify<T: Ord>(buf: &mut [T], root_idx: usize) {
        let (l, r) = Self::child_idx(root_idx);
        let (valid_l, valid_r) = (l < buf.len(), r < buf.len());

        // base case
        if !valid_l && !valid_r {
            return;
        }

        let (larger_child, large_child_idx) = if valid_l && valid_r && buf[l] < buf[r] {
            (&buf[r], r)
        } else {
            (&buf[l], l)
        };

        if *larger_child >= buf[root_idx] {
            buf.swap(root_idx, large_child_idx);

            // recursively fix the affected subtree
            Self::heapify(buf, large_child_idx)
        }
    }

    fn parent_idx(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn child_idx(idx: usize) -> (usize, usize) {
        (2 * idx + 1, 2 * idx + 2)
    }
}

#[cfg(test)]
mod test {
    use super::HeapSort;

    #[test]
    fn test_heapify() {
        let mut buf = vec![1, 7, 3, 2, 9, 27];
        HeapSort::build_max_heap(&mut buf);
        println!("{:?}", &buf);
        for i in 0..buf.len() {
            let (l, r) = HeapSort::child_idx(i);

            if l < buf.len() {
                assert!(buf[i] >= buf[l]);
            }

            if r < buf.len() {
                assert!(buf[i] >= buf[r]);
            }
        }
    }
}
