use criterion::{Criterion, criterion_group, criterion_main};
use std::fmt::Display;
use std::hint::black_box;

use aoc2025::utils;

#[path = "../src/bin/day1/main.rs"]
mod day1;
#[path = "../src/bin/day2/main.rs"]
mod day2;
#[path = "../src/bin/day3/main.rs"]
mod day3;
#[path = "../src/bin/day4/main.rs"]
mod day4;

pub fn benchmark<T: Display, P: Display>(
    crit: &mut Criterion,
    day: utils::Day,
    input: &str,
    first: fn(input: &str) -> T,
    second: fn(input: &str) -> P,
) {
    let day_num = day as i8;

    let mut group = crit.benchmark_group(&format!("day{day_num}"));

    group.bench_function("part1", |benchmark| {
        benchmark.iter(|| first(black_box(input)))
    });

    group.bench_function("part2", |benchmark| {
        benchmark.iter(|| second(black_box(input)))
    });
}

fn day1(crit: &mut Criterion) {
    let details = day1::get_details();
    benchmark(crit, details.day, &details.input, day1::part1, day1::part2);
}

fn day2(crit: &mut Criterion) {
    let details = day2::get_details();
    benchmark(crit, details.day, &details.input, day2::part1, day2::part2);
}

fn day3(crit: &mut Criterion) {
    let details = day3::get_details();
    benchmark(crit, details.day, &details.input, day3::part1, day3::part2);
}

fn day4(crit: &mut Criterion) {
    let details = day4::get_details();
    benchmark(crit, details.day, &details.input, day4::part1, day4::part2);
}

criterion_group!(benches, day1, day2, day3, day4);
criterion_main!(benches);
