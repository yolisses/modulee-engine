use criterion::{criterion_group, criterion_main, Criterion};
use modulee_engine::Graph;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("graph process 10 sine waves", |b| {
        let graph_json = include_str!("benchmark_graph.json");
        b.iter(|| {
            let mut graph = Graph::new();
            graph
                .update_from_json(graph_json)
                .expect("Error setting modules from JSON");

            for pitch in 50..60 {
                graph.set_note_on(pitch as f32);
            }

            for _ in 0..48000 {
                graph.process();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
