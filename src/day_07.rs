use std::io::prelude::*;
extern crate permutator;
extern crate utilities as utils;
use permutator::Permutation;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn part1(int_code_str: &String) -> isize {
	let phase_codes: Vec<isize> = vec![0, 1, 2, 3, 4];
	let all_phase_codes: Vec<Vec<isize>> =
		phase_codes.to_vec().permutation().collect();
	let mut signal = BinaryHeap::new();

	for test_phase_settings in all_phase_codes {
		let mut input_heap: VecDeque<isize> = VecDeque::new();
		let mut int_code: Vec<isize> = int_code_str
			.split(',')
			.map(|op_code_char| op_code_char.parse().unwrap())
			.collect();

		let mut phase = test_phase_settings.iter();
		input_heap.push_back(*phase.next().unwrap());
		input_heap.push_back(0);
		for amp in 1..6 {
			let mut amplifier =
				utils::VmMachine::init(int_code.to_vec(), &mut input_heap);
			amplifier.non_interactive();
			amplifier.bootloader();

			if amp <= 4 {
				let phase_value = *phase.next().unwrap();
				amplifier.diagnostic_input.push_back(phase_value);
			}
			let out_put_value = amplifier.get_output();
			amplifier.diagnostic_input.push_back(out_put_value);

			int_code = amplifier.get_memory();
			signal.push(amplifier.get_output());
		}
	}
	return signal.peek().unwrap().to_owned();
}

#[cfg_attr(test, allow(dead_code))]
fn main() -> std::io::Result<()> {
	let mut int_code_string: String = String::new();
	utils::open_input("./assets/input_day07")
		.read_line(&mut int_code_string)?;

	println!("Results for Day 7");
	println!("-------------------");
	println!("Part 1: {}", part1(&int_code_string));

	Ok(())
}
