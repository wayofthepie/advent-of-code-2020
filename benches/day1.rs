use std::{fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group
        .warm_up_time(Duration::from_secs(120))
        .significance_level(0.1)
        .sample_size(200)
        .measurement_time(Duration::from_secs(120));

    let s = fs::read_to_string("resources/day1.txt").expect("file");
    group.bench_function("part_one_solution", |b| {
        b.iter(|| aoc2020::day1::part_one_solution(&s, 2020))
    });
    group.bench_function("part_two_solution", |b| {
        b.iter(|| aoc2020::day1::part_two_solution(&s, 2020))
    });

    let worst = fs::read_to_string("resources/day1_worst.txt").expect("file");
    group.bench_function("part_two_solution worse case input", |b| {
        b.iter(|| aoc2020::day1::part_two_solution(black_box(&worst), 2000003))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
