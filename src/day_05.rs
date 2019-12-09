use std::io::prelude::*;
extern crate utilities as utils;
use std::collections::VecDeque;

fn part1_and_2(int_code_str: &String, interactive: bool) -> String {
	let int_code: Vec<isize> = int_code_str
		.split(',')
		.map(|op_code_char| op_code_char.parse().unwrap())
		.collect();

	let mut dummy_heap = VecDeque::new();
	let mut int_code_computer =
		utils::VmMachine::init(int_code, &mut dummy_heap);
	if !interactive {
		int_code_computer.non_interactive();
	}
	int_code_computer.bootloader();

	return int_code_computer.get_logger();
}

#[cfg_attr(test, allow(dead_code))]
fn main() -> std::io::Result<()> {
	let mut int_code_string: String = String::new();
	utils::open_input("./assets/input_day05")
		.read_line(&mut int_code_string)?;

	println!("Results for Day 5");
	println!("-------------------");
	println!("Provide, first input: 1");
	println!("Provide, second input: 5");
	println!(
		"Part 1: diagnostic output\n {}",
		part1_and_2(&int_code_string, true)
	);
	println!(
		"Part 2: diagnostic output\n {}",
		part1_and_2(&int_code_string, true)
	);

	Ok(())
}
