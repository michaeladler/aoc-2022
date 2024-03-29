use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_aoc(c: &mut Criterion) {
    // marker
    c.bench_function("day25", |b| b.iter(|| black_box(aoc::solve(25))));
    c.bench_function("day24", |b| b.iter(|| black_box(aoc::solve(24))));
    c.bench_function("day23", |b| b.iter(|| black_box(aoc::solve(23))));
    c.bench_function("day22", |b| b.iter(|| black_box(aoc::solve(22))));
    c.bench_function("day21", |b| b.iter(|| black_box(aoc::solve(21))));
    c.bench_function("day20", |b| b.iter(|| black_box(aoc::solve(20))));
    c.bench_function("day19", |b| b.iter(|| black_box(aoc::solve(19))));
    c.bench_function("day18", |b| b.iter(|| black_box(aoc::solve(18))));
    c.bench_function("day17", |b| b.iter(|| black_box(aoc::solve(17))));
    c.bench_function("day16", |b| b.iter(|| black_box(aoc::solve(16))));
    c.bench_function("day15", |b| b.iter(|| black_box(aoc::solve(15))));
    c.bench_function("day14", |b| b.iter(|| black_box(aoc::solve(14))));
    c.bench_function("day13", |b| b.iter(|| black_box(aoc::solve(13))));
    c.bench_function("day12", |b| b.iter(|| black_box(aoc::solve(12))));
    c.bench_function("day11", |b| b.iter(|| black_box(aoc::solve(11))));
    c.bench_function("day10", |b| b.iter(|| black_box(aoc::solve(10))));
    c.bench_function("day09", |b| b.iter(|| black_box(aoc::solve(9))));
    c.bench_function("day08", |b| b.iter(|| black_box(aoc::solve(8))));
    c.bench_function("day07", |b| b.iter(|| black_box(aoc::solve(7))));
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
