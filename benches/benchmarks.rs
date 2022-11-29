use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_aoc(c: &mut Criterion) {
    // marker
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark_aoc
);
criterion_main!(benches);
