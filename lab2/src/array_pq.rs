use std::collections::VecDeque;

use num_traits::{Bounded, Num};
use sc2001::graph::Edge;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArrayPriorityQueue<T> {
    data: VecDeque<T>,
}

impl<T> From<VecDeque<T>> for ArrayPriorityQueue<T> {
    fn from(data: VecDeque<T>) -> Self {
        Self { data }
    }
}

impl<T> From<Vec<T>> for ArrayPriorityQueue<T> {
    fn from(data: Vec<T>) -> Self {
        let data = VecDeque::from(data);
        Self { data }
    }
}

impl<T, const N: usize> From<[T; N]> for ArrayPriorityQueue<T> {
    fn from(data: [T; N]) -> Self {
        let data = VecDeque::from(data);
        Self { data }
    }
}

impl<T> ArrayPriorityQueue<T> {
    pub fn push(&mut self, item: T) {
        self.data.push_back(item);
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> ArrayPriorityQueue<Edge<T>>
where
    T: Num + Bounded + PartialOrd + Copy,
{
    pub fn pop(&mut self) -> Option<Edge<T>> {
        if self.data.is_empty() {
            return None;
        }

        let mut min = T::max_value();
        let mut i = 0;

        // finds the min value in array
        // keep track of the index of the item
        for (j, Edge(weight, _)) in self.data.iter().enumerate() {
            if *weight <= min {
                min = *weight;
                i = j;
            }
        }

        // swap the first element with index
        self.data.swap(0, i);

        // then pop front item
        self.data.pop_front()
    }
}

impl ArrayPriorityQueue<u32> {
    pub fn pop(&mut self) -> Option<u32> {
        if self.data.is_empty() {
            return None;
        }

        let mut min = u32::MAX;
        let mut i = 0;

        // finds the min value in array
        // keep track of the index of the item
        for (j, weight) in self.data.iter().enumerate() {
            if *weight <= min {
                min = *weight;
                i = j;
            }
        }

        // swap the first element with index
        self.data.swap(0, i);

        // then pop front item
        self.data.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use sc2001::graph::Edge;

    use super::ArrayPriorityQueue;

    #[test]
    fn get_min() {
        let mut array_pq = ArrayPriorityQueue::from(vec![21389, 12389, 1, 1239, 2139]);
        let smallest = array_pq.pop();
        assert_eq!(smallest, Some(1));
    }

    #[test]
    fn get_min_pair() {
        let mut array_pq =
            ArrayPriorityQueue::from(vec![Edge(21389, 0), Edge(12389, 1), Edge(1, 2), Edge(1239, 3), Edge(2139, 4)]);
        let smallest = array_pq.pop();
        assert_eq!(smallest, Some(Edge(1, 2)));
    }

    #[test]
    fn get_min_same() {
        let mut array_pq = ArrayPriorityQueue::from(vec![1, 1, 1, 1, 1]);
        let smallest = array_pq.pop();
        assert_eq!(smallest, Some(1));
    }

    #[test]
    fn get_min_same_pair() {
        let mut array_pq = ArrayPriorityQueue::from(vec![Edge(1, 0), Edge(1, 1), Edge(1, 2), Edge(1, 3), Edge(1, 4)]);
        let smallest = array_pq.pop();
        assert_eq!(smallest, Some(Edge(1, 4)));
    }

    #[test]
    fn get_min_empty() {
        let mut array_pq = ArrayPriorityQueue::<u32>::from(vec![]);
        let smallest = array_pq.pop();
        assert_eq!(smallest, None);
    }

    #[test]
    fn get_min_one() {
        let mut array_pq = ArrayPriorityQueue::from(vec![1]);
        let smallest = array_pq.pop();
        assert_eq!(smallest, Some(1));
    }

    #[test]
    fn test_push() {
        let mut array_pq = ArrayPriorityQueue::from(vec![1]);
        let array_pq_2 = ArrayPriorityQueue::from(vec![1, 2]);
        array_pq.push(2);
        assert_eq!(array_pq, array_pq_2);
    }
}
