mod graph;
mod array_pq;

#[cfg(test)]
mod tests {
    use crate::graph::{MatrixGraph, ListGraph};

    #[test]
    fn clone_graph() {
        let mat = vec![
            vec![0, 10, 5, 0, 0],
            vec![0, 0, 2, 1, 0],
            vec![0, 3, 0, 9, 2],
            vec![0, 0, 0, 0, 4],
            vec![0, 0, 0, 6, 0],
        ];

        let mat_graph = MatrixGraph::from(mat.clone());        
        let list_graph = ListGraph::from(mat_graph.clone());
        let mat_g = MatrixGraph::from(list_graph);        
        assert_eq!(mat_graph, mat_g);
    }
}
