#[derive(Debug, Clone)]
pub struct ArrayPriorityQueue<T> {
    data: Vec<T>,
}

impl<T> From<Vec<T>> for ArrayPriorityQueue<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl ArrayPriorityQueue<i32> {
    pub fn pop(&self) -> Option<i32> {
        let mut max = i32::MAX;
        Some(3)
    }
}
