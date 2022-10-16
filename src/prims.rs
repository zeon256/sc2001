use std::collections::BinaryHeap;

use crate::graph::{AdjList, Edge, Graph};

#[derive(Debug, Clone)]
pub struct MstInfo<T> {
    pub vertexes: Vec<Edge<T>>,
    pub cost: T,
}

impl<T> MstInfo<T> {
    pub fn new(vertexes: Vec<Edge<T>>, cost: T) -> Self {
        Self { vertexes, cost }
    }
}

/// Minimum spanning tree : Connect all vertexes without any cycle
pub fn prims(graph: Graph<AdjList<Edge<u32>>>, src: usize) -> Option<MstInfo<u32>> {
    let mut visited = vec![false; graph.len()];
    let mut pq = BinaryHeap::from([Edge(0u32, src)]);

    let mut path = vec![];
    let mut cost = 0;

    while !pq.is_empty() && path.len() <= graph.len() {
        let Edge(curr_weight, curr_vertex) = pq.pop().unwrap();

        if !visited[curr_vertex] {
            path.push(Edge(curr_weight, curr_vertex));
            visited[curr_vertex] = true;
            cost += curr_weight;
        }

        for Edge(weight, src_vertex) in graph.neighbours(curr_vertex) {
            if !visited[*src_vertex] {
                pq.push(Edge(*weight, *src_vertex));
            }
        }
    }

    if path.len() < graph.len() {
        return None;
    }

    Some(MstInfo::new(path, cost))
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::{AdjList, AdjMatrix, Edge, Graph},
        prims::prims,
    };

    use super::MstInfo;

    #[test]
    pub fn test_mst_struct() {
        let mst = MstInfo::new(
            vec![
                Edge(1, 0),
                Edge(2, 9),
                Edge(28, 3),
                Edge(12, 12),
                Edge(123, 1),
                Edge(123, 12),
            ],
            21,
        );
        println!("{:?}", mst);
    }

    #[test]
    pub fn prims_test() {
        type ListGraph = Graph<AdjList<Edge<u32>>>;

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
        assert_eq!(mst_info.cost, 21);
    }

    #[test]
    pub fn prims_test_no_mst() {
        type ListGraph = Graph<AdjList<Edge<u32>>>;

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
        type ListGraph = Graph<AdjList<Edge<u32>>>;

        let adj_list = [
            vec![Edge(2, 1)],
            vec![Edge(2, 0)],
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
}
