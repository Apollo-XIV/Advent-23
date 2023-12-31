use advent_23::days::day_06::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn part_1_bench(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(|| part_1()));
}

pub fn part_2_bench(c: &mut Criterion) {
    c.bench_function("part 2", |b| b.iter(|| part_2()));
}
criterion_group!(benches, part_1_bench, part_2_bench);
criterion_main!(benches);
