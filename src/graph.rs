use num_traits::Zero;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph<T> {
    internal_repr: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdjMatrix<T>(Vec<Vec<T>>);

#[derive(Debug, Clone, PartialEq)]
pub struct AdjList<T>(Vec<Vec<T>>);

impl<T> Graph<AdjMatrix<T>>
where
    T: Copy + Zero + PartialEq,
{
    pub fn neighbours(&self, vertex: usize) -> Vec<(usize, T)> {
        let mut buf = vec![];
        for (i, x) in self.internal_repr.0[vertex].iter().enumerate() {
            if *x != T::zero() {
                buf.push((i, *x))
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
        assert!(data.len() == data[0].len());
        Self {
            internal_repr: AdjMatrix(data),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{AdjMatrix, Graph};

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
}
