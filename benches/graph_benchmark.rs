use criterion::{criterion_group, criterion_main, Criterion};
use modulee_engine::graph::Graph;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("graph process 10 sine waves", |b| {
        let groups_json = include_str!("benchmark_graph.json");
        b.iter(|| {
            let mut graph = Graph::new();
            graph
                .set_groups_from_json(groups_json)
                .expect("Error setting groups from JSON");
            graph.set_main_group_id(1837259400);

            for pitch in 50..60 {
                graph.set_note_on(pitch as f32);
            }

            for _ in 0..44100 {
                graph.process();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
