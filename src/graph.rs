use std::{cmp::Ordering, fmt::Debug};

use num_traits::Zero;

#[derive(Clone, PartialEq)]
pub struct Graph<T> {
    pub internal_repr: T,
}

impl<T: Debug> Debug for Graph<AdjMatrix<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graph").field("internal_repr", &self.internal_repr).finish()
    }
}

impl<T: Debug> Debug for Graph<AdjList<T>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("ListGraph\n");

        for (i, t) in self.internal_repr.0.iter().enumerate() {
            s.push_str(&format!("{i} -> "));
            for (j, k) in t.iter().enumerate() {
                s.push_str(&format!("{:?}", *k));
                if j != t.len() - 1 {
                    s.push_str(&format!(" -> "));
                }
            }
            s.push('\n');
        }

        f.write_str(s.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Edge<T>(pub T, pub usize);

impl PartialOrd for Edge<u32> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(Ordering::Greater) => return Some(Ordering::Less),
            Some(Ordering::Less) => return Some(Ordering::Greater),
            None => {}
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
            None => {}
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
pub struct AdjList<T>(pub Vec<Vec<T>>);

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

    #[inline]
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

impl<T, const N: usize> From<[Vec<T>; N]> for Graph<AdjList<T>>
where
    T: Clone,
{
    fn from(data: [Vec<T>; N]) -> Self {
        Self {
            internal_repr: AdjList(data.to_vec()),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Graph<AdjMatrix<T>> {
    fn from(data: Vec<Vec<T>>) -> Self {
        if data.len() == 0 {
            return Self {
                internal_repr: AdjMatrix(data),
            };
        }

        assert!(data.len() == data[0].len());
        Self {
            internal_repr: AdjMatrix(data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{AdjMatrix, Edge, Graph};

    #[test]
    fn ui_test() {
        let graph_adj_matrix =
            Graph::<AdjMatrix<i32>>::from(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]]);

        let x = graph_adj_matrix.neighbours(1);
        dbg!(x);
    }

    #[test]
    fn ordering() {
        let (e, e1) = (Edge(2u32, 2), Edge(3, 4));
        assert!(e > e1);
    }
}
