use aoc_2020::day1;
use aoc_2020::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::BufReader;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../inputs/day01_input.txt");
    let d1 = day1::Input::read(BufReader::new(input.as_bytes())).unwrap();
    c.bench_function("day 1, parse", |b| {
        b.iter(|| <day1::FirstStep as Solution>::Input::read(BufReader::new(input.as_bytes())))
    });
    c.bench_function("day 1, step 1", |b| {
        b.iter(|| day1::FirstStep::solve(black_box(d1.clone())))
    });
    c.bench_function("day 1, parse", |b| {
        b.iter(|| <day1::SecondStep as Solution>::Input::read(BufReader::new(input.as_bytes())))
    });
    c.bench_function("day 1, step 2", |b| {
        b.iter(|| day1::SecondStep::solve(black_box(d1.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
