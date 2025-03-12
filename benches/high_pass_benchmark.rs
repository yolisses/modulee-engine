use criterion::{criterion_module, criterion_main, Criterion};
use modulee_engine::filter::high_pass::HighPass;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("high-pass with static inputs", |b| {
        b.iter(|| {
            let mut high_pass = HighPass::default();
            for i in 0..48000 {
                let input = i as f32 % 1.;
                let frequency = i as f32 % 1. + 10.;
                let resonance = i as f32 % 1. + 0.1;
                high_pass.process(input, frequency, resonance);
            }
        })
    });
}

criterion_module!(benches, criterion_benchmark);
criterion_main!(benches);
