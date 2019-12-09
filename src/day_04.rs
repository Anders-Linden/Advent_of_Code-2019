use std::collections::HashMap;

fn convert_to_digits(numbers: &i32) -> Vec<u32> {
	numbers
		.to_string()
		.chars()
		.map(|d| d.to_digit(10).unwrap())
		.collect()
}

fn increasing_digit(digits: &Vec<u32>) -> bool {
	digits
		.iter()
		.zip(digits.iter().skip(1))
		.all(|(first, second)| first <= second)
}
fn double_digits(digits: &Vec<u32>) -> bool {
	digits
		.iter()
		.zip(digits.iter().skip(1))
		.any(|(first, second)| first == second)
}

fn strict_doubles(digits: &Vec<u32>) -> bool {
	let mut occurrence: HashMap<u32, u32> = HashMap::new();
	for digit in digits {
		*occurrence.entry(*digit).or_insert(0) += 1;
	}
	for (_digit, value) in &occurrence {
		if *value == 2 {
			return true;
		}
	}
	false
}

pub fn part1(input: std::ops::Range<i32>) -> usize {
	let passwords: Vec<Vec<u32>> = (input)
		.map(|x| convert_to_digits(&x))
		.filter(|digits| increasing_digit(digits))
		.filter(|digits| double_digits(digits))
		.collect();

	passwords.len()
}

pub fn part2(input: std::ops::Range<i32>) -> usize {
	let tes2: Vec<Vec<u32>> = (input.clone())
		.map(|x| convert_to_digits(&x))
		.filter(|digits| increasing_digit(digits))
		.filter(|digits| strict_doubles(digits))
		.collect();
	tes2.len()
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
	println!("Results for Day 4");
	println!("-------------------");
	println!("Part 1: {}", part1(264_793..803_935));
	println!("Part 2: {}", part2(264_793..803_935));
}

#[cfg(test)]
mod tests {
	#[cfg_attr(test, allow(unused_imports))]
	use super::*;
	#[test]
	fn code_coverage_test_main() {
		assert_eq!(main(), ());
	}
}
