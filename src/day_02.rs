use std::io::prelude::*;
extern crate utilities as utils;

fn instruction_pipeline(int_code: &mut Vec<u32>) {
	for pc in 0..int_code.len() as usize {
		if pc % 4 == 0 {
			match int_code[pc as usize] {
				99 => break,
				1 => {
					let edit_pos = int_code[pc + 3] as usize;
					let value_one_position = int_code[pc + 1] as usize;
					let value_two_position = int_code[pc + 2] as usize;
					int_code[edit_pos] = int_code[value_one_position]
						+ int_code[value_two_position];
				}
				2 => {
					let edit_pos = int_code[pc + 3] as usize;
					let value_one_position = int_code[pc + 1] as usize;
					let value_two_position = int_code[pc + 2] as usize;
					int_code[edit_pos] = int_code[value_one_position]
						* int_code[value_two_position];
				}
				_ => {}
			}
		}
	}
}

pub fn part1(int_code_str: &String) -> u32 {
	let mut int_code: Vec<u32> = int_code_str
		.split(',')
		.map(|op_code_char| op_code_char.parse().unwrap())
		.collect();

	// Fire fix
	int_code[1] = 12;
	int_code[2] = 2;

	instruction_pipeline(&mut int_code);
	return int_code[0];
}

pub fn part2(int_code_str: &String) -> String {
	let int_code: Vec<u32> = int_code_str
		.split(',')
		.map(|op_code_char| op_code_char.parse().unwrap())
		.collect();
	for noun in 0..99 {
		for verb in 0..99 {
			let mut test_code = int_code.to_vec();
			test_code[1] = noun;
			test_code[2] = verb;
			instruction_pipeline(&mut test_code);

			if test_code[0] == 19690720 {
				return format!(
					"100 * {} + {} = {}",
					noun,
					verb,
					100 * noun + verb
				);
			}
		}
	}
	return format!("Error");
}

fn main() -> std::io::Result<()> {
	let mut int_code_string: String = String::new();
	utils::open_input("./assets/input_day02")
		.read_line(&mut int_code_string)?;

	println!("Results for Day 2");
	println!("-------------------");
	println!("Part 1: {}", part1(&int_code_string));
	println!("Part 2: {}", part2(&int_code_string));

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn code_coverage_test_main() {
		assert_eq!(main().unwrap(), ());
	}
	#[test]
	fn op_code_test_change_op() {
		let mut test_vec = vec![1, 0, 0, 0, 99];
		instruction_pipeline(&mut test_vec);
		assert_eq!(test_vec, vec![2, 0, 0, 0, 99]);
	}
	#[test]
	fn op_test_change_pointer() {
		let mut test_vec = vec![2, 3, 0, 3, 99];
		instruction_pipeline(&mut test_vec);
		assert_eq!(test_vec, vec![2, 3, 0, 6, 99]);
	}
	#[test]
	fn op_test_change_eol() {
		let mut test_vec = vec![2, 4, 4, 5, 99, 0];
		instruction_pipeline(&mut test_vec);
		assert_eq!(test_vec, vec![2, 4, 4, 5, 99, 9801]);
	}

	#[test]
	fn op_test_multiple_instructions() {
		let mut test_vec = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
		instruction_pipeline(&mut test_vec);
		assert_eq!(test_vec, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
	}
}
