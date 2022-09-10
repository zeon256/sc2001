pub struct HeapSort;

impl HeapSort {
    pub fn sort<T: Ord + Copy>(mut buf: &mut [T]) {
        let sz = buf.len();
        // Self::make_max_heap(buf);
        for i in 0..buf.len() {
            Self::heapify(buf, 0);
            buf.swap(0, sz-1);
            buf = &mut buf[..sz];
        }
    }

    // pub fn make_max_heap<T: Ord + Copy>(buf: &mut [T]) {}

    pub fn heapify<T: Ord + Copy>(buf: &mut [T], root_idx: usize) {
    }
}