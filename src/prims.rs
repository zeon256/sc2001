use crate::graph::{AdjList, Edge, Graph};
use std::{collections::BinaryHeap, cmp::Reverse};

type ListGraph = Graph<AdjList<Edge<u32>>>;
#[derive(Debug, Clone)]
pub struct MstInfo<T> {
    pub predecessors: Vec<Option<usize>>,
    pub distance: Vec<T>,
    pub visited: Vec<bool>,
    pub min_cost: T,
}

impl<T> MstInfo<T> {
    pub fn new(
        predecessors: Vec<Option<usize>>,
        visited: Vec<bool>,
        distance: Vec<T>,
        min_cost: T,
    ) -> Self {
        Self {
            predecessors,
            visited,
            distance,
            min_cost,
        }
    }
}

/// Minimum spanning tree : Connect all vertexes without any cycle
pub fn prims(graph: ListGraph, src: usize) -> Option<MstInfo<u32>> {
    // S
    let mut visited = vec![false; graph.len()];
    let mut predecessors = vec![None::<usize>; graph.len()];
    let mut distance = vec![u32::MAX; graph.len()];

    // minimising priority queue
    let mut pq = BinaryHeap::from([Reverse((0, src, None))]);
    let mut no_visited = 0;
    let mut min_cost = 0;
    distance[src] = 0;

    while !pq.is_empty() && no_visited <= graph.len() {
        let Reverse((weight, vertex, predecessor)) = pq.pop().unwrap();

        if visited[vertex] {
            continue;
        }

        // #[cfg(debug_assertions)]
        // println!("Adding: {:?}", Edge(weight, vertex));

        visited[vertex] = true;
        min_cost += weight;
        no_visited += 1;
        predecessors[vertex] = predecessor;
        if let Some(predecessor_dist_idx) = predecessor {
            distance[vertex] = distance[predecessor_dist_idx] + weight;
        }

        for Edge(weight, neighbour) in graph.neighbours(vertex) {
            if !visited[*neighbour] {
                pq.push(Reverse((*weight, *neighbour, Some(vertex))));
                // println!("Setting predecessor for: {} to {}", *neighbour, vertex);
            }
        }
    }

    if no_visited != graph.len() {
        return None;
    }

    Some(MstInfo::new(predecessors, visited, distance, min_cost))
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{AdjList, AdjMatrix, Edge, Graph},
        prims::{prims, ListGraph},
    };

    use super::MstInfo;

    #[test]
    pub fn test_mst_struct() {
        let mst = MstInfo::new(vec![], vec![], vec![], 21);
        println!("{:?}", mst);
    }

    #[test]
    pub fn prims_test() {
        let adj_list = [
            vec![Edge(10, 1), Edge(1, 2), Edge(4, 3)],
            vec![Edge(10, 0), Edge(3, 2), Edge(1, 3)],
            vec![Edge(1, 0), Edge(2, 3), Edge(8, 5), Edge(3, 1)],
            vec![Edge(4, 0), Edge(2, 2), Edge(2, 5), Edge(7, 6)],
            vec![Edge(1, 1), Edge(1, 5), Edge(8, 7)],
            vec![Edge(8, 2), Edge(2, 3), Edge(6, 6), Edge(9, 7), Edge(1, 4)],
            vec![Edge(7, 3), Edge(6, 5), Edge(12, 7)],
            vec![Edge(12, 6), Edge(9, 5), Edge(8, 4)],
        ];

        let graph = ListGraph::from(adj_list);
        println!("{:?}", &graph);
        let mst_info = prims(graph, 0).unwrap();
        println!("{:?}", mst_info);
        assert_eq!(mst_info.min_cost, 21);
    }

    #[test]
    pub fn prims_test_graph2() {
        let adj_list = [
            vec![Edge(1, 1), Edge(7, 2)],
            vec![Edge(1, 0), Edge(5, 2), Edge(4, 3), Edge(3, 4)],
            vec![Edge(7, 0), Edge(5, 1), Edge(6, 4)],
            vec![Edge(4, 1), Edge(2, 4)],
            vec![Edge(2, 3), Edge(6, 2)],
        ];

        let graph = ListGraph::from(adj_list);
        println!("{:?}", &graph);
        let mst_info = prims(graph, 0).unwrap();
        println!("{:?}", mst_info);
    }

    #[test]
    pub fn prims_test_no_mst() {
        let adj_list = [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];

        let graph = ListGraph::from(adj_list);
        println!("{:?}", &graph);
        let mst_info = prims(graph, 0);
        match mst_info {
            Some(_) => panic!("Should be None!"),
            None => {}
        };
    }

    #[test]
    pub fn prims_disconnected() {
        let adj_list = [vec![Edge(2, 1)], vec![Edge(2, 0)], vec![]];

        let graph = ListGraph::from(adj_list);
        println!("{:?}", &graph);
        let mst_info = prims(graph, 0);
        match mst_info {
            Some(_) => panic!("Should be None!"),
            None => {}
        };
    }
}
