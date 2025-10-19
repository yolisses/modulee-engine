use criterion::{criterion_group, criterion_main, Criterion};
use modulee_engine::Graph;

// Unfortunately, the system is not fast enough to run 10 notes on 48000 samples
// per second 100 times in 5 seconds
pub fn criterion_benchmark(c: &mut Criterion) {
    let graph_json = include_str!("piano_benchmark.json");

    c.bench_function("process 5 notes with piano", |b| {
        b.iter(|| {
            let mut graph = Graph::new(10000.);
            graph
                .update_from_json(graph_json)
                .expect("Error setting modules from JSON");

            for pitch in 50..55 {
                graph.set_note_on(pitch as f32);
            }

            for _ in 0..10000 {
                graph.process();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
