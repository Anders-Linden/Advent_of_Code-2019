use std::fs::File;
use std::io::BufReader;
use std::io;
use std::collections::VecDeque;

pub fn open_input(file_path: &str) -> std::io::BufReader<std::fs::File> {
	let file = match File::open(file_path) {
		Err(why) => panic!("couldn't open, {}", why),
		Ok(file) => file,
	};
	BufReader::new(file)
}


pub struct VmMachine<'a> {
	memory: Vec<isize>,
	program_counter: usize,
	logger: String,
	auto: bool,
	pub diagnostic_input: &'a mut VecDeque<isize>,
	diagnostic_output: isize
}

impl<'a> VmMachine<'a> {
	pub fn init(memory: Vec<isize>, input_heap: &'a mut VecDeque<isize>) -> Self {
		Self {
			memory,
			program_counter: 0,
			logger: "".to_string(),
			auto: false,
			diagnostic_input: input_heap,
			diagnostic_output: 0
		}
	}

	pub fn bootloader(&mut self) {
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
		self.diagnostic_output = self.get(address);
		self.step(2);
	}
	pub fn get_output(&self) -> isize {
		return self.diagnostic_output;
	}

	fn diagnostic_store(&mut self) {
		let address = self.mem_address(1);

		if self.auto {
			let value = self.diagnostic_input.pop_front().unwrap();
			self.set(address, value);
		} else {
			println!("Diagnostic input: ");
			self.set(address, Self::stdin());
		}
		self.step(2);
	}
	pub fn get_memory(&self) -> Vec<isize> {
		return self.memory.to_vec()
	}

	pub fn get_logger(&self) -> String {
		return self.logger.to_string();
	}
	#[allow(dead_code)]
	pub fn non_interactive(&mut self) {
		// Used for automated tests and benchmarks
		self.auto = true;
	}

	fn logger(&mut self, data: isize) {
		self.logger.push_str(&format!("{:?}\n", data).to_string());
	}
}