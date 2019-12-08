use std::io::prelude::*;
extern crate utilities as utils;
use std::io;

struct VmMachine {
	memory: Vec<isize>,
	program_counter: usize,
	logger: String,
	in_benchmark: bool,
}

impl VmMachine {
	fn init(memory: Vec<isize>) -> Self {
		Self {
			memory,
			program_counter: 0,
			logger: "".to_string(),
			in_benchmark: false,
		}
	}

	fn bootloader(&mut self) {
		loop {
			match self.instruction() {
				1 => self.addition(),
				2 => self.multiplication(),
				3 => self.diagnostic_store(),
				4 => self.diagnostic_read(),
				5 => self.jump_if_true(),
				6 => self.jump_if_false(),
				7 => self.less_than(),
				8 => self.equal(),
				99 => return,
				_ => panic!("Unexpected instruction"),
			};
		}
	}

	fn instruction(&self) -> isize {
		self.get(self.program_counter) % 100
	}

	fn step(&mut self, count: usize) {
		self.program_counter =
			(self.program_counter + count) % self.memory.len();
	}

	fn get(&self, position: usize) -> isize {
		self.memory[position]
	}

	fn set(&mut self, position: usize, value: isize) {
		self.memory[position] = value;
	}

	fn mem_address(&self, position: usize) -> usize {
		let memory_position =
			self.program_counter as usize + position % self.memory.len();
		self.memory[memory_position] as usize
	}

	fn param(&self, position: usize) -> isize {
		let memory_position =
			self.program_counter as usize + position % self.memory.len();

		let mode = (self.memory[self.program_counter]
			/ 10isize.pow(1 + position as u32))
			% 2;
		if mode == 0 {
			self.get(self.memory[memory_position] as usize)
		} else {
			self.memory[memory_position]
		}
	}
	fn addition(&mut self) {
		let value_one = self.param(1);
		let value_two = self.param(2);
		let address = self.mem_address(3);

		self.set(address, value_one + value_two);
		self.step(4);
	}

	fn multiplication(&mut self) {
		let value_one = self.param(1);
		let value_two = self.param(2);
		let address = self.mem_address(3);

		self.set(address, value_one * value_two);
		self.step(4);
	}

	fn jump_if_true(&mut self) {
		if self.param(1) != 0 {
			self.program_counter = self.param(2) as usize;
		} else {
			self.step(3);
		}
	}
	fn jump_if_false(&mut self) {
		if self.param(1) == 0 {
			self.program_counter = self.param(2) as usize;
		} else {
			self.step(3);
		}
	}

	fn less_than(&mut self) {
		let address = self.mem_address(3);

		if self.param(1) < self.param(2) {
			self.set(address, 1);
		} else {
			self.set(address, 0);
		}

		self.step(4);
	}

	fn equal(&mut self) {
		let address = self.mem_address(3);

		if self.param(1) == self.param(2) {
			self.set(address, 1);
		} else {
			self.set(address, 0);
		}

		self.step(4);
	}

	fn stdin() -> isize {
		let mut buffer = String::new();
		io::stdin()
			.read_line(&mut buffer)
			.expect("STDIN read failed.");
		buffer.trim().parse::<isize>().unwrap()
	}

	fn diagnostic_read(&mut self) {
		let address = self.mem_address(1);
		self.logger(self.get(address));
		self.step(2);
	}

	fn diagnostic_store(&mut self) {
		let address = self.mem_address(1);
		println!("Diagnostic input: ");
		if self.in_benchmark {
			self.set(address, 1);
		} else {
			self.set(address, Self::stdin());
		}
		self.step(2);
	}

	#[allow(dead_code)]
	fn non_interactive(&mut self) {
		// Used for automated tests and benchmarks
		self.in_benchmark = true;
	}

	fn logger(&mut self, data: isize) {
		self.logger.push_str(&format!("{:?}\n", data).to_string());
	}
}

fn part1_and_2(int_code_str: &String, interactive: bool) -> String {
	let int_code: Vec<isize> = int_code_str
		.split(',')
		.map(|op_code_char| op_code_char.parse().unwrap())
		.collect();

	let mut int_code_computer = VmMachine::init(int_code);
	if !interactive {
		int_code_computer.non_interactive();
	}
	int_code_computer.bootloader();

	return int_code_computer.logger;
}

fn main() -> std::io::Result<()> {
	let mut int_code_string: String = String::new();
	utils::open_input("./assets/input_day05")
		.read_line(&mut int_code_string)?;

	println!("Results for Day 5");
	println!("-------------------");
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
