use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_aoc(c: &mut Criterion) {
    // marker
    c.bench_function("day06", |b| b.iter(|| black_box(aoc::solve(6))));
    c.bench_function("day05", |b| b.iter(|| black_box(aoc::solve(5))));
    c.bench_function("day04", |b| b.iter(|| black_box(aoc::solve(4))));
    c.bench_function("day03", |b| b.iter(|| black_box(aoc::solve(3))));
    c.bench_function("day02", |b| b.iter(|| black_box(aoc::solve(2))));
    c.bench_function("day01", |b| b.iter(|| black_box(aoc::solve(1))));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = benchmark_aoc
);
criterion_main!(benches);
