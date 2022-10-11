use std::cmp::Ordering;

use num_traits::Zero;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph<T> {
    pub internal_repr: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Edge<T>(pub T, pub usize);

impl PartialOrd for Edge<u32> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(Ordering::Greater) => return Some(Ordering::Less),
            Some(Ordering::Less) => return Some(Ordering::Greater),
            None => {},
        }
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Edge<u32> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for Edge<u64> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(Ordering::Greater) => return Some(Ordering::Less),
            Some(Ordering::Less) => return Some(Ordering::Greater),
            None => {},
        }
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for Edge<u64> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdjMatrix<T>(pub Vec<Vec<T>>);

#[derive(Debug, Clone, PartialEq)]
pub struct AdjList<T>(Vec<Vec<T>>);

impl<T> Graph<AdjMatrix<T>>
where
    T: Copy + Zero + PartialEq,
{
    pub fn neighbours(&self, vertex: usize) -> Vec<Edge<T>> {
        let mut buf = vec![];
        for (i, x) in self.internal_repr.0[vertex].iter().enumerate() {
            if *x != T::zero() {
                buf.push(Edge(*x, i))
            }
        }

        buf
    }

    pub fn len(&self) -> usize {
        self.internal_repr.0.len()
    }
}

impl<T> Graph<AdjList<T>> {
    pub fn neighbours(&self, vertex: usize) -> &[T] {
        self.internal_repr.0[vertex].as_slice()
    }

    pub fn neighbours_mut(&mut self, vertex: usize) -> &mut [T] {
        self.internal_repr.0[vertex].as_mut_slice()
    }

    pub fn len(&self) -> usize {
        self.internal_repr.0.len()
    }
}

impl<T> From<Vec<Vec<T>>> for Graph<AdjList<T>> {
    fn from(data: Vec<Vec<T>>) -> Self {
        Self {
            internal_repr: AdjList(data),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Graph<AdjMatrix<T>> {
    fn from(data: Vec<Vec<T>>) -> Self {
        if data.len() == 0 {
            return Self {
                internal_repr: AdjMatrix(data)
            }
        }
        
        assert!(data.len() == data[0].len());
        Self {
            internal_repr: AdjMatrix(data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{AdjMatrix, Graph, Edge};

    #[test]
    fn ui_test() {
        let graph_adj_matrix = Graph::<AdjMatrix<i32>>::from(vec![
            vec![1, 0, 0],
            vec![0, 1, 1],
            vec![0, 1, 1],
        ]);

        let x = graph_adj_matrix.neighbours(1);
        dbg!(x);
    }

    #[test]
    fn ordering() {
        let (e, e1) = (Edge(2u32, 2), Edge(3, 4));
        assert!(e > e1);
    }
}
