extern crate criterion;
use std::io::prelude::*;
extern crate utilities as utils;
use criterion::{criterion_group, criterion_main, Criterion, Benchmark};

#[path = "../src/day_01.rs"]
mod day_01;

#[path = "../src/day_02.rs"]
mod day_02;

#[path = "../src/day_04.rs"]
mod day_04;

fn criterion_benchmark(c: &mut Criterion) {

	let mut day_1_data: Vec<u32> = Vec::new();
	for line in utils::open_input("./assets/input_day01").lines() {
		day_1_data.push(line.unwrap().parse().unwrap())
	}

	let day_1_data1 = day_1_data.clone();
	c.bench(
		"Day 1",
		Benchmark::new("Part 1", move |b| {
			b.iter(|| day_01::part1(&day_1_data.clone()))
		})
		.with_function("Part 2", move |b| {
			b.iter(|| day_01::part2(&day_1_data1.clone()))
		})
		.sample_size(10)
		.measurement_time(std::time::Duration::new(10, 0)),
	);

	let mut day_2_data: String = String::new();
	utils::open_input("./assets/input_day02")
		.read_line(&mut day_2_data)
		.unwrap();

	let day_2_data1 = day_2_data.clone();
	c.bench(
		"Day 2",
		Benchmark::new("Part 1", move |b| {
			b.iter(|| day_02::part1(&day_2_data.clone()))
		})
		.with_function("Part 2", move |b| {
			b.iter(|| day_02::part2(&day_2_data1.clone()))
		})
		.sample_size(10)
		.measurement_time(std::time::Duration::new(10, 0)),
	);

	c.bench(
		"Day 4",
		Benchmark::new("Part 1", |b| {
			b.iter(|| day_04::part1(264_793..803_935))
		})
		.with_function("Part 2", |b| b.iter(|| day_04::part2(264_793..803_935)))
		.sample_size(10)
		.measurement_time(std::time::Duration::new(10, 0)),
	);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
