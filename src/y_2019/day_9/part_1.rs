use std::collections::HashMap;

pub fn solve(input_text: &String) {
	let program = input_text
		.split(|c| c == '\n' || c == ',')
		.filter_map(|c| c.parse::<isize>().ok())
		.collect::<Vec<isize>>();
	let mut memory = HashMap::<usize, isize>::new();
	for (adrs, num) in program.iter().enumerate() {
		memory.insert(adrs, num.clone());
	}
	run_program(memory);
}

pub fn run_program(mut memory: HashMap<usize, isize>) {
	let mut program_counter = 0;
	let mut relative_base_offset = 0;
	loop {
		let instruction_header = memory.get(&program_counter).unwrap();
		let opcode = instruction_header % 100;
		match opcode {
			1 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				let operand_two = get_operand(
					&memory,
					program_counter + 2,
					relative_base_offset,
					instruction_header.clone(),
					2,
				);
				let sum = operand_one + operand_two;
				let store_at = get_operand(
					&memory,
					program_counter + 3,
					relative_base_offset,
					instruction_header.clone(),
					3,
				) as usize;
				memory.insert(store_at, sum);
				program_counter += 4;
			}
			2 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				let operand_two = get_operand(
					&memory,
					program_counter + 2,
					relative_base_offset,
					instruction_header.clone(),
					2,
				);
				let product = operand_one * operand_two;
				let store_at = get_operand(
					&memory,
					program_counter + 3,
					relative_base_offset,
					instruction_header.clone(),
					3,
				) as usize;
				memory.insert(store_at, product);
				program_counter += 4;
			}
			3 => {
				// Takes Input
				let input_value = 1;
				let store_at = get_operand(
					&memory,
					program_counter + 3,
					relative_base_offset,
					instruction_header.clone(),
					3,
				) as usize;
				memory.insert(store_at, input_value);
				program_counter += 2;
			}
			4 => {
				// Writes Output
				let output_value = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				println!("Output: {}", output_value);
				program_counter += 2;
			}
			5 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				if operand_one == 0 {
					program_counter += 3;
					continue;
				}
				let operand_two = get_operand(
					&memory,
					program_counter + 2,
					relative_base_offset,
					instruction_header.clone(),
					2,
				);
				program_counter = operand_two as usize;
			}
			6 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				if operand_one != 0 {
					program_counter += 3;
					continue;
				}
				let operand_two = get_operand(
					&memory,
					program_counter + 2,
					relative_base_offset,
					instruction_header.clone(),
					2,
				);
				program_counter = operand_two as usize;
			}
			7 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				let operand_two = get_operand(
					&memory,
					program_counter + 2,
					relative_base_offset,
					instruction_header.clone(),
					2,
				);
				let store_at = get_operand(
					&memory,
					program_counter + 3,
					relative_base_offset,
					instruction_header.clone(),
					3,
				) as usize;
				if operand_one < operand_two {
					memory.insert(store_at, 1);
				} else {
					memory.insert(store_at, 0);
				}
				program_counter += 4;
			}
			8 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				let operand_two = get_operand(
					&memory,
					program_counter + 2,
					relative_base_offset,
					instruction_header.clone(),
					2,
				);
				let store_at = get_operand(
					&memory,
					program_counter + 3,
					relative_base_offset,
					instruction_header.clone(),
					3,
				) as usize;
				if operand_one == operand_two {
					memory.insert(store_at, 1);
				} else {
					memory.insert(store_at, 0);
				}
				program_counter += 4;
			}
			9 => {
				let operand_one = get_operand(
					&memory,
					program_counter + 1,
					relative_base_offset,
					instruction_header.clone(),
					1,
				);
				relative_base_offset = relative_base_offset + operand_one;
				program_counter += 2;
			}
			99 => {
				// Halt
				println!("HLT");
				break;
			}
			_catch_all => {
				// Error
				panic!("The opcode '{_catch_all}' is not supported.");
			}
		}
	}
}

fn get_operand(
	memory: &HashMap<usize, isize>,
	program_counter: usize,
	relative_base_offset: isize,
	instruction_header: isize,
	operand_number: usize,
) -> isize {
	let parameter_mode = instruction_header.to_digits().iter().nth_back(operand_number + 1).unwrap_or(&0).clone();
	match parameter_mode {
		0 => {
			let a = memory.get(&program_counter).unwrap_or(&0).clone();
			if operand_number == 3 {
				return a;
			}
			let b = memory.get(&(a as usize)).unwrap_or(&0).clone();
			b
		}
		1 => {
			let a = memory.get(&program_counter).unwrap_or(&0).clone();
			a
		}
		2 => {
			let a = memory.get(&program_counter).unwrap_or(&0).clone();
			let b = (relative_base_offset + a) as usize;
			if operand_number == 3 {
				return b as isize;
			}
			let c = memory.get(&b).unwrap_or(&0).clone();
			c
		}
		_ => {
			panic!("Only the following parameter mode are supported: 0, 1, 2.");
		}
	}
}

trait ToDigits {
	fn to_digits(&self) -> Vec<i8>;
}

impl ToDigits for isize {
	fn to_digits(&self) -> Vec<i8> {
		if *self == 0 {
			return vec![0];
		}
		let mut n = *self;
		let mut digits = Vec::new();
		while n > 0 {
			digits.push((n % 10) as i8);
			n /= 10;
		}
		digits.reverse();
		digits
	}
}
