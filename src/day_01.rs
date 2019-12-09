use std::io::prelude::*;
extern crate utilities as utils;

fn calc_fuel(mass: i32) -> i32 {
	(mass / 3 - 2)
}

fn calc_fuel_mass(mass: u32, sum: u32) -> u32 {
	let result = calc_fuel(mass as i32);
	if result > 0 {
		calc_fuel_mass(result as u32, sum + result as u32)
	} else {
		sum
	}
}

pub fn part1(data: &Vec<u32>) -> u32 {
	let total_fuel: u32 = data
		.into_iter()
		.map(|x| calc_fuel(x.clone() as i32) as u32)
		.sum();
	return total_fuel;
}

pub fn part2(data: &Vec<u32>) -> u32 {
	let total_fuel: u32 =
		data.into_iter().map(|x| calc_fuel_mass(*x, 0) as u32).sum();

	return total_fuel;
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
	let mut data: Vec<u32> = Vec::new();
	for line in utils::open_input("./assets/input_day01").lines() {
		data.push(line.unwrap().parse().unwrap())
	}

	println!("Results for Day 1");
	println!("-------------------");
	println!("Part 1: {}", part1(&data));
	println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
	#[cfg_attr(test, allow(unused_imports))]
	use super::*;

	#[test]
	fn code_coverage_test_main() {
		assert_eq!(main(), ());
	}

	#[test]
	fn fuel() {
		assert_eq!(calc_fuel(1969), 654);
		assert_eq!(calc_fuel(100756), 33583);
	}
	#[test]
	fn fuel_mass() {
		assert_eq!(calc_fuel_mass(1969, 0), 966);
		assert_eq!(calc_fuel_mass(100756, 0), 50346);
	}
}
