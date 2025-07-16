use std::fs;

pub fn solve() {
	let input_text = fs::read_to_string("src/y_2019/day_5/input.txt").unwrap();
	let program =
		input_text.split(|c| c == '\n' || c == ',').filter_map(|c| c.parse::<isize>().ok()).collect::<Vec<isize>>();
	run_program(program);
}

pub fn run_program(mut program: Vec<isize>) {
	let mut program_counter = 0;
	loop {
		let instruction_header = program[program_counter];
		let opcode = instruction_header % 100;
		match opcode {
			1 => {
				// Add
				let parameter_modes = instruction_header.to_digits();
				let operand_one: isize;
				if *parameter_modes.iter().nth_back(2).unwrap_or(&0) == 0 {
					operand_one = program[program[program_counter + 1] as usize];
				} else {
					operand_one = program[program_counter + 1];
				}
				let operand_two: isize;
				if *parameter_modes.iter().nth_back(3).unwrap_or(&0) == 0 {
					operand_two = program[program[program_counter + 2] as usize];
				} else {
					operand_two = program[program_counter + 2];
				}
				let sum = operand_one + operand_two;
				let output_pos = program[program_counter + 3];
				program[output_pos as usize] = sum;
				program_counter += 4;
			}
			2 => {
				// Multiply
				let parameter_modes = instruction_header.to_digits();
				let operand_one: isize;
				if *parameter_modes.iter().nth_back(2).unwrap_or(&0) == 0 {
					operand_one = program[program[program_counter + 1] as usize];
				} else {
					operand_one = program[program_counter + 1];
				}
				let operand_two: isize;
				if *parameter_modes.iter().nth_back(3).unwrap_or(&0) == 0 {
					operand_two = program[program[program_counter + 2] as usize];
				} else {
					operand_two = program[program_counter + 2];
				}
				let product = operand_one * operand_two;
				let output_pos = program[program_counter + 3];
				program[output_pos as usize] = product;
				program_counter += 4;
			}
			3 => {
				// Takes Input
				let input_value = 1;
				let output_pos = program[program_counter + 1];
				program[output_pos as usize] = input_value;
				program_counter += 2;
			}
			4 => {
				// Writes Output
				let output_value = program[program[program_counter + 1] as usize];
				println!("Output: {}", output_value);
				program_counter += 2;
			}
			99 => {
				// Halt
				println!("HLT");
				break;
			}
			_ => {
				// Error
			}
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
