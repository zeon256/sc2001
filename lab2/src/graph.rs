use std::ops::Deref;
use sc2001::graph::{Graph, AdjMatrix, AdjList};


#[derive(Debug, Copy, Clone)]
pub struct Edge<T> {
    pub vertex: usize,
    pub weight: T,
}

impl<T> Edge<T> {
    pub fn new(vertex: usize, weight: T) -> Self {
        Self { vertex, weight }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixGraph(pub Graph<AdjMatrix<i32>>);

#[derive(Debug, Clone)]
pub struct ListGraph(pub Graph<AdjList<Edge<i32>>>);

impl From<Vec<Vec<i32>>> for MatrixGraph {
    fn from(matrix: Vec<Vec<i32>>) -> Self {
        Self(Graph::from(matrix))
    }
}

impl From<Vec<Vec<Edge<i32>>>> for ListGraph {
    fn from(list: Vec<Vec<Edge<i32>>>) -> Self {
        Self(Graph::from(list))
    }
}

impl Deref for MatrixGraph {
    type Target = Graph<AdjMatrix<i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for ListGraph {
    type Target = Graph<AdjList<Edge<i32>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MatrixGraph> for ListGraph {
    fn from(graph: MatrixGraph) -> Self {
        let sz = graph.len();
        let mut list = vec![vec![]; sz];

        for i in 0..sz {
            for (j, weight) in graph.neighbours(i).into_iter() {
                list[i].push(Edge::new(j, weight));
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
            for edge in graph.0.neighbours(i).iter() {
                mat[i][edge.vertex] = edge.weight;
            }
        }
        
        Self::from(mat)
    }
}