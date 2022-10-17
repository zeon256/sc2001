use crate::{
    graph::{AdjList, Edge, Graph},
    union_find::{UnionFind, WeightedQuickUnion},
};

type ListGraph = Graph<AdjList<Edge<u32>>>;
type KruskalRunInfo = (UnionFind<WeightedQuickUnion, usize, Vec<usize>>, u32);

pub fn kruskal(graph: ListGraph) -> Option<KruskalRunInfo> {
    // use DSU with Weighted QuickUnion 
    let mut uf = UnionFind::<WeightedQuickUnion, usize, Vec<usize>>::new(graph.len());
    // store triplet (weight, from_vertex, to_vertex)
    let mut edges = vec![];
    // no of edges so far, there will be a total |V| - 1 edges for MST
    let mut no_edges = 0;
    // min cost
    let mut cost = 0;

    // convert info of graph to triplets and store in edges array
    for from in 0..graph.len() {
        for Edge(weight, to) in graph.neighbours(from) {
            edges.push((*weight, from, *to));
        }
    }

    // we sort the by weights which is the first element in teh triplet
    // nlgn
    edges.sort_unstable();
    
    // since we alr have the sorted edges by weight
    // we just connect them tgt using DSU
    for (weight, from, to) in edges {
        // we have already found the MST
        if no_edges == graph.len() - 1 {
            break;
        }

        // if connected => there is a cycle => we dont care and go next
        if !uf.connected(to, from) {
            println!("Unionizing : {} and {} with weight {}", to, from, weight);
            uf.union(to, from);
            cost += weight;
            no_edges += 1;
        }
    }

    // means there is no MST, ie. there exist one vertex that is not connected
    if no_edges != graph.len() - 1 {
        return None;
    }

    Some((uf, cost))
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::Edge,
        kruskal::{kruskal, ListGraph},
    };

    #[test]
    fn test_kruskal() {
        let adj_list = [
            vec![Edge(10, 1), Edge(1, 2), Edge(4, 3)],
            vec![Edge(10, 0), Edge(3, 2), Edge(1, 4)],
            vec![Edge(1, 0), Edge(2, 3), Edge(8, 5), Edge(3, 1)],
            vec![Edge(4, 0), Edge(2, 2), Edge(2, 5), Edge(7, 6)],
            vec![Edge(1, 1), Edge(1, 5), Edge(8, 7)],
            vec![Edge(8, 2), Edge(2, 3), Edge(6, 6), Edge(9, 7), Edge(1, 4)],
            vec![Edge(7, 3), Edge(6, 5), Edge(12, 7)],
            vec![Edge(12, 6), Edge(9, 5), Edge(8, 4)],
        ];

        let graph = ListGraph::from(adj_list);
        println!("{:?}", &graph);
        let (uf, cost) = kruskal(graph).unwrap();
        assert_eq!(21, cost);
    }

    #[test]
    fn test_kruskal_graph2() {
        let adj_list = [
            vec![Edge(1, 1), Edge(7, 2)],
            vec![Edge(1, 0), Edge(5, 2), Edge(4, 3), Edge(3, 4)],
            vec![Edge(7, 0), Edge(5, 1), Edge(6, 4)],
            vec![Edge(4, 1), Edge(2, 4)],
            vec![Edge(2, 3), Edge(6, 2)],
        ];

        let graph = ListGraph::from(adj_list);
        println!("{:?}", &graph);
        let (uf, cost) = kruskal(graph).unwrap();
        
        assert_eq!(11, cost);
    }

    #[test]
    fn test_kruskal_graph3() {
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
        if let Some(_) = kruskal(graph){
            panic!("there shouldnt have any MST");
        }
    }
}
