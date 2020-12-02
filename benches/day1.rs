use std::{fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sample-size-example");
    group
        .warm_up_time(Duration::from_secs(10))
        .significance_level(0.1)
        .sample_size(10)
        .measurement_time(Duration::from_secs(10));

    let s = fs::read_to_string("resources/day1.txt").expect("file");
    group.bench_function("part_one_solution", |b| {
        b.iter(|| aoc2020::day1::part_one_solution(&s, 2020))
    });
    group.bench_function("part_two_solution single binsearch", |b| {
        b.iter(|| aoc2020::day1::part_two_solution_single_binsearch(&s, 2020))
    });

    group.bench_function("part_two_solution 2d binsearch", |b| {
        b.iter(|| aoc2020::day1::part_two_solution_2d_binsearch(&s, 2020))
    });

    let worst = fs::read_to_string("resources/day1_worst2.txt").expect("file");
    group.bench_function("part_two_solution single binsearch large data", |b| {
        b.iter(|| aoc2020::day1::part_two_solution_single_binsearch(black_box(&worst), 20000003))
    });

    group.bench_function("part_two_solution 2d binsearch large data", |b| {
        b.iter(|| aoc2020::day1::part_two_solution_2d_binsearch(black_box(&worst), 20000003))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
