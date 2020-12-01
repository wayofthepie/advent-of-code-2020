use std::{fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group
        .warm_up_time(Duration::from_secs(30))
        .significance_level(0.1)
        .sample_size(500)
        .measurement_time(Duration::from_secs(30));

    let s = fs::read_to_string("resources/day1.txt").expect("");
    group.bench_function("part_one_solution", |b| {
        b.iter(|| aoc2020::day1::part_one_solution(black_box(&s)))
    });
    group.bench_function("part_two_solution", |b| {
        b.iter(|| aoc2020::day1::part_two_solution(black_box(&s)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
