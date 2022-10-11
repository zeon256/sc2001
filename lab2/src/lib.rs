#![allow(dead_code)]

use std::collections::BinaryHeap;

use array_pq::ArrayPriorityQueue;
use graph::{ListGraph, MatrixGraph};
use nanorand::{Rng, WyRand};
use sc2001::graph::Edge;

pub mod array_pq;
pub mod graph;

#[derive(Debug, Clone)]
pub struct DjikstraRunInfo {
    pub distance: Vec<u32>,
    pub predecessors: Vec<Option<usize>>,
    pub visited: Vec<bool>,
}

impl DjikstraRunInfo {
    pub fn new(distance: Vec<u32>, predecessors: Vec<Option<usize>>, visited: Vec<bool>) -> Self {
        Self {
            distance,
            predecessors,
            visited,
        }
    }
}

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
pub fn djikstra_bheap_list_graph(graph: ListGraph, src: usize) -> DjikstraRunInfo {
    if graph.len() == 0 {
        return DjikstraRunInfo::new(vec![], vec![], vec![]);
    }

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
    DjikstraRunInfo::new(distance, predecessors, visited)
}

#[rustfmt::skip]
pub fn djikstra_bheap_matrix(graph: MatrixGraph, src: usize) -> DjikstraRunInfo {
    if graph.len() == 0 {
        return DjikstraRunInfo::new(vec![], vec![], vec![]);
    }

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
    
    DjikstraRunInfo::new(distance, predecessors, visited)
}

#[rustfmt::skip]
pub fn djikstra_array_pq_list_graph(graph: ListGraph, src: usize) -> DjikstraRunInfo {
    if graph.len() == 0 {
        return DjikstraRunInfo::new(vec![], vec![], vec![]);
    }

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
    
    DjikstraRunInfo::new(distance, predecessors, visited)
}

#[rustfmt::skip]
pub fn djikstra_array_pq_matrix(graph: MatrixGraph, src: usize) -> DjikstraRunInfo {
    if graph.len() == 0 {
        return DjikstraRunInfo::new(vec![], vec![], vec![]);
    }

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
    
    DjikstraRunInfo::new(distance, predecessors, visited)
}

pub fn gen_graph<T>(seed: T, v: usize, e: usize) -> MatrixGraph
where
    T: Into<Option<u64>>,
{
    assert!(e <= usize::pow(v, 2));
    let mut graph = vec![vec![0; v]; v];

    let mut rng = match seed.into() {
        Some(seed) => WyRand::new_seed(seed),
        None => WyRand::new(),
    };

    let mut curr = 0;

    while curr < e {
        let (i, j) = (rng.generate_range(0..v), rng.generate_range(0..v));
        if i == j {
            continue;
        }
        if graph[i][j] != 0 {
            continue;
        }
        graph[i][j] = rng.generate_range(1..100);
        curr += 1;
    }

    // for i in &graph {
    //     print!("[ ");
    //     for j in i {
    //         print!("{} ", *j);
    //     }
    //     print!(" ]\n");
    // }

    MatrixGraph::from(graph)
}

pub fn assert_graph_edge(x: &MatrixGraph, e: usize) {
    let mut cnt = 0;
    for i in 0..x.len() {
        for j in 0..x.len() {
            if x.0.internal_repr.0[i][j] != 0 {
                cnt += 1;
            }
        }
    }
    assert_eq!(cnt, e);
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{Read, Write}};

    use sc2001::Estimates;

    use crate::{
        assert_graph_edge, djikstra_array_pq_list_graph, djikstra_array_pq_matrix,
        djikstra_bheap_list_graph, djikstra_bheap_matrix, gen_graph,
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

    fn mat4() -> (Vec<Vec<u32>>, Vec<u32>) {
        (vec![], vec![])
    }

    fn all_graphs() -> Vec<(Vec<Vec<u32>>, Vec<u32>)> {
        vec![mat1(), mat2(), mat3(), mat4()]
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
            assert_eq!(res, dist.distance);
        }
    }

    #[test]
    fn test_djikstra_bheap_list() {
        for (mat, res) in all_graphs() {
            let mat_graph = ListGraph::from(MatrixGraph::from(mat));
            let dist = djikstra_bheap_list_graph(mat_graph, 0);
            assert_eq!(res, dist.distance);
        }
    }

    #[test]
    fn test_djikstra_array_pq_list() {
        for (mat, res) in all_graphs() {
            let mat_graph = ListGraph::from(MatrixGraph::from(mat));
            let dist = djikstra_array_pq_list_graph(mat_graph, 0);
            assert_eq!(res, dist.distance);
        }
    }

    #[test]
    fn test_djikstra_array_pq_matrix() {
        for (mat, res) in all_graphs() {
            let mat_graph = MatrixGraph::from(mat);
            let dist = djikstra_array_pq_matrix(mat_graph, 0);

            assert_eq!(res, dist.distance);
        }
    }

    #[test]
    fn generate_graph() {
        let x = gen_graph(42069, 20, 20);
        assert_graph_edge(&x, 20);
        let x = gen_graph(42069, 10, 20);
        assert_graph_edge(&x, 20);
        let x = gen_graph(42069, 5, 20);
        assert_graph_edge(&x, 20);
    }

    #[ignore]
    #[test]
    fn compile_density_test() {
        let mut all_estimates = String::from(
                "e,v,bheap_list_graph_mean,bheap_matrix_graph_mean,array_pq_list_graph_mean,array_pq_matrix_graph\n");
        for e in (100..=9900).step_by(100) {
            let v = 100;

            let benches = [
                format!("djikstra_bheap_list_graph(e_{},v_{})", e, v),
                format!("djikstra_bheap_matrix_graph(e_{},v_{})", e, v),
                format!("djikstra_array_pq_list_graph(e_{},v_{})", e, v),
                format!("djikstra_array_pq_matrix_graph(e_{},v_{})", e, v),
            ];

            all_estimates.push_str(&format!("{e},{v},"));

            for b in benches {
                let mut buf = String::new();
                let file_name = format!("density_test/criterion/{b}/new/estimates.json");
                dbg!(&file_name);
                let mut f = File::open(file_name).unwrap();
                let _ = f.read_to_string(&mut buf).unwrap();
                let estimates = serde_json::from_str::<Estimates>(&buf).unwrap();
                let mean = estimates.mean.point_estimate / 1000.0;
                all_estimates.push_str(&format!("{mean},"));
            }
            all_estimates.push_str("\n");
        }
        
        let mut density_test = File::create("density_test.csv").unwrap();
        density_test.write_all(all_estimates.as_bytes()).unwrap();
    }
    
    #[ignore]
    #[test]
    fn compile_stress_test() {
        let mut all_estimates = String::from(
                "e,v,bheap_list_graph_mean,bheap_matrix_graph_mean,array_pq_list_graph_mean,array_pq_matrix_graph\n");
                
        for v in (0..=1000).step_by(20) {

            let e = usize::pow(v, 2) - v;
            let benches = [
                format!("complete_graph_djikstra_bheap_list_graph(e_{},v_{})", e, v),
                format!("complete_graph_djikstra_bheap_matrix_graph(e_{},v_{})", e, v),
                format!("complete_graph_djikstra_array_pq_list_graph(e_{},v_{})", e, v),
                format!("complete_graph_djikstra_array_pq_matrix_graph(e_{},v_{})", e, v),
            ];

            all_estimates.push_str(&format!("{e},{v},"));

            for b in benches {
                let mut buf = String::new();
                let file_name = format!("stress_test/criterion/{b}/new/estimates.json");
                dbg!(&file_name);
                let mut f = File::open(file_name).unwrap();
                let _ = f.read_to_string(&mut buf).unwrap();
                let estimates = serde_json::from_str::<Estimates>(&buf).unwrap();
                let mean = estimates.mean.point_estimate / 1000.0;
                all_estimates.push_str(&format!("{mean},"));
            }
            
            all_estimates.push_str("\n");
        }
        
        let mut density_test = File::create("stress_test.csv").unwrap();
        density_test.write_all(all_estimates.as_bytes()).unwrap();
    }
}
