#![allow(dead_code)]

use std::collections::BinaryHeap;

use array_pq::ArrayPriorityQueue;
use graph::{ListGraph, MatrixGraph};
use nanorand::{Rng, WyRand};
use sc2001::graph::Edge;

pub mod array_pq;
pub mod graph;

fn djikstra_setup<T>(sz_v: usize, src: usize) -> (Vec<u32>, Vec<Option<usize>>, Vec<bool>, T)
where
    T: From<[Edge<u32>; 1]>,
{
    // initialise distance to every other node as u32::MAX
    // which is functionally the same as infinity
    let mut distance = vec![u32::MAX; sz_v];
    let predecessors = vec![None::<usize>; sz_v];
    let visited = vec![false; sz_v];

    // construct initial priority queue from src's neighbours
    // where u32 = weight, usize = index
    let pq = T::from([Edge(0, src)]);

    // src distance to itself is 0
    distance[src] = 0;

    (distance, predecessors, visited, pq)
}

#[rustfmt::skip]
fn djikstra_bheap_list_graph(graph: ListGraph, src: usize) -> Vec<u32> {
    let (
        mut distance,
        mut predecessors,
        mut visited,
        mut pq
    ) = djikstra_setup::<BinaryHeap<_>>(graph.len(), src);

    while !pq.is_empty() {
        // get cheapest vertex
        // SAFETY: unwrap is ok because queue not empty
        let Edge(_, vertex) = pq.pop().unwrap();

        // visit curr vertex
        visited[vertex] = true;

        // get vertex neighbours and update distance table from vertex
        // only visit vertex that have not been visited
        for Edge(weight, neighbour) in graph.neighbours(vertex) {
            // update distance table and predecessors
            if !visited[*neighbour] && distance[*neighbour] > distance[vertex] + weight {
                distance[*neighbour] = distance[vertex] + weight;
                predecessors[*neighbour] = Some(vertex);
                pq.push(Edge(distance[*neighbour], *neighbour));
            }
        }
    }
    distance
}

#[rustfmt::skip]
fn djikstra_bheap_matrix(graph: MatrixGraph, src: usize) -> Vec<u32> {
    let (
        mut distance,
        mut predecessors,
        mut visited,
        mut pq
    ) = djikstra_setup::<BinaryHeap<_>>(graph.len(), src);

    while !pq.is_empty() {
        // get cheapest vertex
        // SAFETY: unwrap is ok because queue not empty
        let Edge(_, vertex) = pq.pop().unwrap();

        // visit curr vertex
        visited[vertex] = true;

        // get vertex neighbours and update distance table from vertex
        // only visit vertex that have not been visited
        for Edge(weight, neighbour) in &graph.neighbours(vertex) {
            // update distance table and predecessors
            if !visited[*neighbour] && distance[*neighbour] > distance[vertex] + weight {
                distance[*neighbour] = distance[vertex] + weight;
                predecessors[*neighbour] = Some(vertex);
                pq.push(Edge(distance[*neighbour], *neighbour));
            }
        }
    }
    distance
}

#[rustfmt::skip]
fn djikstra_array_pq_list_graph(graph: ListGraph, src: usize) -> Vec<u32> {
    let (
        mut distance,
        mut predecessors,
        mut visited,
        mut pq
    ) = djikstra_setup::<ArrayPriorityQueue<_>>(graph.len(), src);

    while !pq.is_empty() {
        // get cheapest vertex
        // SAFETY: unwrap is ok because queue not empty
        let Edge(_, vertex) = pq.pop().unwrap();

        // visit curr vertex
        visited[vertex] = true;

        // get vertex neighbours and update distance table from vertex
        // only visit vertex that have not been visited
        for Edge(weight, neighbour) in graph.neighbours(vertex) {
            // update distance table and predecessors
            if !visited[*neighbour] && distance[*neighbour] > distance[vertex] + weight {
                distance[*neighbour] = distance[vertex] + weight;
                predecessors[*neighbour] = Some(vertex);
                pq.push(Edge(distance[*neighbour], *neighbour));
            }
        }
    }
    distance
}

#[rustfmt::skip]
fn djikstra_array_pq_matrix(graph: MatrixGraph, src: usize) -> Vec<u32> {
    let (
        mut distance,
        mut predecessors,
        mut visited,
        mut pq
    ) = djikstra_setup::<ArrayPriorityQueue<_>>(graph.len(), src);

    while !pq.is_empty() {
        // get cheapest vertex
        // SAFETY: unwrap is ok because queue not empty
        let Edge(_, vertex) = pq.pop().unwrap();

        // visit curr vertex
        visited[vertex] = true;

        // get vertex neighbours and update distance table from vertex
        // only visit vertex that have not been visited
        for Edge(weight, neighbour) in &graph.neighbours(vertex) {
            // update distance table and predecessors
            if !visited[*neighbour] && distance[*neighbour] > distance[vertex] + weight {
                distance[*neighbour] = distance[vertex] + weight;
                predecessors[*neighbour] = Some(vertex);
                pq.push(Edge(distance[*neighbour], *neighbour));
            }
        }
    }
    distance
}

/// A graph with “closer to” |V| edges is considered sparse
pub fn gen_sparse_graph<T>(seed: T, n: usize) -> MatrixGraph
where
    T: Into<Option<u64>>,
{
    let seed = seed.into();
    let mut graph = vec![vec![0; n]; n];
    let mut rng = match seed.clone() {
        Some(seed) => WyRand::new_seed(seed),
        None => WyRand::new(),
    };

    let mut rng_2 = match seed.clone() {
        Some(seed) => WyRand::new_seed(seed + 69),
        None => WyRand::new(),
    };

    let mut rng_3 = match seed.clone() {
        Some(seed) => WyRand::new_seed(seed + 69123),
        None => WyRand::new(),
    };

    for _ in 0..n {
        // generate random indexes
        let (i, j) = (rng.generate_range(0..n), rng_2.generate_range(0..n));
        // dont allow self loops
        if i == j {
            continue;
        }
        graph[i][j] = rng_3.generate_range(0..100);
    }

    for i in &graph {
        print!("[ ");
        for j in i {
            print!("{} ", *j);
        }
        print!(" ]\n");
    }

    MatrixGraph::from(graph)
}

pub fn gen_complete_graph<T>(seed: T, n: usize) -> MatrixGraph
where
    T: Into<Option<u64>>,
{
    let mut graph = vec![vec![0; n]; n];
    let mut rng = match seed.into() {
        Some(seed) => WyRand::new_seed(seed),
        None => WyRand::new(),
    };

    for (i, item) in graph.iter_mut().enumerate() {
        for (j, item) in item.iter_mut().enumerate() {
            if i == j {
                continue;
            }
            *item = rng.generate_range(0..100);
        }
    }

    for i in &graph {
        print!("[ ");
        for j in i {
            print!("{} ", *j);
        }
        print!(" ]\n");
    }

    MatrixGraph::from(graph)
}

#[cfg(test)]
mod tests {
    use crate::{
        djikstra_array_pq_list_graph, djikstra_array_pq_matrix, djikstra_bheap_list_graph,
        djikstra_bheap_matrix, gen_complete_graph, gen_sparse_graph,
        graph::{ListGraph, MatrixGraph},
    };

    fn mat1() -> (Vec<Vec<u32>>, Vec<u32>) {
        (
            vec![
                vec![0, 10, 5, 0, 0],
                vec![0, 0, 2, 1, 0],
                vec![0, 3, 0, 9, 2],
                vec![0, 0, 0, 0, 4],
                vec![0, 0, 0, 6, 0],
            ],
            vec![0, 8, 5, 9, 7],
        )
    }

    fn mat2() -> (Vec<Vec<u32>>, Vec<u32>) {
        (
            vec![vec![0, 24, 0], vec![1, 0, 0], vec![0, 0, 0]],
            vec![0, 24, u32::MAX],
        )
    }

    fn mat3() -> (Vec<Vec<u32>>, Vec<u32>) {
        (
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            vec![0, u32::MAX, u32::MAX],
        )
    }

    fn all_graphs() -> Vec<(Vec<Vec<u32>>, Vec<u32>)> {
        vec![mat1(), mat2(), mat3()]
    }

    #[test]
    fn clone_graph() {
        let mat = mat1().0;
        let mat_graph = MatrixGraph::from(mat.clone());
        let list_graph = ListGraph::from(mat_graph.clone());
        let mat_g = MatrixGraph::from(list_graph);
        assert_eq!(mat_graph, mat_g);
    }

    #[test]
    fn test_djikstra_bheap_matrix() {
        for (mat, res) in all_graphs() {
            let mat_graph = MatrixGraph::from(mat);
            let dist = djikstra_bheap_matrix(mat_graph, 0);
            assert_eq!(res, dist);
        }
    }

    #[test]
    fn test_djikstra_bheap_list() {
        for (mat, res) in all_graphs() {
            let mat_graph = ListGraph::from(MatrixGraph::from(mat));
            let dist = djikstra_bheap_list_graph(mat_graph, 0);
            assert_eq!(res, dist);
        }
    }

    #[test]
    fn test_djikstra_array_pq_list() {
        for (mat, res) in all_graphs() {
            let mat_graph = ListGraph::from(MatrixGraph::from(mat));
            let dist = djikstra_array_pq_list_graph(mat_graph, 0);
            assert_eq!(res, dist);
        }
    }

    #[test]
    fn test_djikstra_array_pq_matrix() {
        for (mat, res) in all_graphs() {
            let mat_graph = MatrixGraph::from(mat);
            let dist = djikstra_array_pq_matrix(mat_graph, 0);

            assert_eq!(res, dist);
        }
    }

    #[test]
    fn test_generate_sparse_graph() {
        gen_sparse_graph(42069, 20);
    }

    #[test]
    fn test_generate_complete_graph() {
        gen_complete_graph(42069, 20);
    }
}
