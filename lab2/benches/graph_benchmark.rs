use criterion::{criterion_group, criterion_main, Criterion};
use lab2::{
    djikstra_bheap_list_graph, gen_graph,
    graph::{ListGraph, MatrixGraph}, assert_graph_edge, djikstra_bheap_matrix, djikstra_array_pq_list_graph, djikstra_array_pq_matrix,
};
use mimalloc::MiMalloc;
use nanorand::{Rng, WyRand};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn criterion_benchmark(c: &mut Criterion) {
    // 9900 because need minus v
    for e in (100..=9900).step_by(100) {
        let v = 100;
        let matrix_graph = gen_graph(42069, v, e);
        let list_graph = ListGraph::from(matrix_graph.clone());
        assert_graph_edge(&matrix_graph, e);

        c.bench_function(
            &format!("djikstra_bheap_list_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || list_graph.clone(),
                    |graph| djikstra_bheap_list_graph(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );

        c.bench_function(
            &format!("djikstra_bheap_matrix_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || matrix_graph.clone(),
                    |graph| djikstra_bheap_matrix(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );

        c.bench_function(
            &format!("djikstra_array_pq_list_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || list_graph.clone(),
                    |graph| djikstra_array_pq_list_graph(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );

        c.bench_function(
            &format!("djikstra_array_pq_matrix_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || matrix_graph.clone(),
                    |graph| djikstra_array_pq_matrix(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );
    }
    
    for v in (0..=1000).step_by(20) {
        // need minus the diagonal
        let e = usize::pow(v, 2) - v;
        let matrix_graph = gen_graph(42069,v, e);
        let list_graph = ListGraph::from(matrix_graph.clone());
        assert_graph_edge(&matrix_graph, e);

        c.bench_function(
            &format!("complete_graph_djikstra_bheap_list_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || list_graph.clone(),
                    |graph| djikstra_bheap_list_graph(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );

        c.bench_function(
            &format!("complete_graph_djikstra_bheap_matrix_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || matrix_graph.clone(),
                    |graph| djikstra_bheap_matrix(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );

        c.bench_function(
            &format!("complete_graph_djikstra_array_pq_list_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || list_graph.clone(),
                    |graph| djikstra_array_pq_list_graph(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );

        c.bench_function(
            &format!("complete_graph_djikstra_array_pq_matrix_graph(e_{},v_{})", e, v),
            |b| {
                b.iter_batched(
                    || matrix_graph.clone(),
                    |graph| djikstra_array_pq_matrix(graph, 0),
                    criterion::BatchSize::SmallInput
                );
            },
        );
    }
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.05).sample_size(30);
    targets = criterion_benchmark
}
criterion_main!(benches);
