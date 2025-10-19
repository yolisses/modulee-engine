use criterion::{criterion_group, criterion_main, Criterion};
use modulee_engine::Graph;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("process 10 notes with piano", |b| {
        let graph_json = include_str!("piano_benchmark.json");
        b.iter(|| {
            let mut graph = Graph::new(48000.);
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
