use sc2001::graph::{AdjList, AdjMatrix, Graph, Edge};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixGraph(pub Graph<AdjMatrix<u32>>);

#[derive(Debug, Clone)]
pub struct ListGraph(pub Graph<AdjList<Edge<u32>>>);

impl From<Vec<Vec<u32>>> for MatrixGraph {
    fn from(matrix: Vec<Vec<u32>>) -> Self {
        Self(Graph::from(matrix))
    }
}

impl From<Vec<Vec<Edge<u32>>>> for ListGraph {
    fn from(list: Vec<Vec<Edge<u32>>>) -> Self {
        Self(Graph::from(list))
    }
}

impl Deref for MatrixGraph {
    type Target = Graph<AdjMatrix<u32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ListGraph {
    type Target = Graph<AdjList<Edge<u32>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MatrixGraph> for ListGraph {
    fn from(graph: MatrixGraph) -> Self {
        let sz = graph.len();
        let mut list = vec![vec![]; sz];

        for i in 0..sz {
            for Edge(weight, j) in graph.neighbours(i).into_iter() {
                list[i].push(Edge(weight, j));
            }
        }

        ListGraph(Graph::from(list))
    }
}

impl From<ListGraph> for MatrixGraph {
    fn from(graph: ListGraph) -> Self {
        let sz = graph.len();
        let mut mat = vec![vec![0; sz]; sz];

        for i in 0..sz {
            for Edge(weight, vertex) in graph.0.neighbours(i).iter() {
                mat[i][*vertex] = *weight;
            }
        }

        Self::from(mat)
    }
}