use num_traits::Zero;

pub struct UndirectedGraph<T> {
    internal_repr: T,
}

pub struct AdjMatrix<T>(Vec<Vec<T>>);
pub struct AdjList<T>(Vec<Vec<T>>);

impl<T> UndirectedGraph<AdjMatrix<T>>
where
    T: Copy + Zero + PartialEq,
{
    pub fn neighours(&self, vertex: usize) -> Vec<T> {
        let mut buf = vec![];
        for x in &self.internal_repr.0[vertex] {
            if *x != T::zero()  && *x != *x {
                buf.push(*x)
            }
        }

        buf
    }
}

impl<T> UndirectedGraph<AdjList<T>>
where
    T:  Clone + Zero,
{
    pub fn neighours(&self, vertex: usize) -> &[T] {
        self.internal_repr.0[vertex].as_slice()
    }

    pub fn neighours_mut(&mut self, vertex: usize) -> &mut[T] {
        self.internal_repr.0[vertex].as_mut_slice()
    }
}

impl<T> From<Vec<Vec<T>>> for UndirectedGraph<AdjList<T>> {
    fn from(data: Vec<Vec<T>>) -> Self {
        Self {
            internal_repr: AdjList(data)
        }
    }
}

impl<T> From<Vec<Vec<T>>> for UndirectedGraph<AdjMatrix<T>> {
    fn from(data: Vec<Vec<T>>) -> Self {
        assert!(data.len() == data[0].len());
        Self {
            internal_repr: AdjMatrix(data)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{UndirectedGraph, AdjMatrix};

    #[test]
    fn ui_test() {
        let graph_adj_matrix = UndirectedGraph::<AdjMatrix<i32>>::from(
            vec![
                vec![1, 0, 0],
                vec![0, 1, 1],
                vec![0, 1, 1],
            ]
        );

        let x = graph_adj_matrix.neighours(1);
        dbg!(x);
    }
}
