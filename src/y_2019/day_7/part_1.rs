pub fn solve(input_text: &String) -> isize {
	let program = input_text
		.split(|c| c == '\n' || c == ',')
		.filter_map(|c| c.parse::<isize>().ok())
		.collect::<Vec<isize>>();
	let mut highest_signal = 0;
	let mut input_signal = 0;
	let mut output = 0;
	for a in 0..=4 {
		for b in 0..=4 {
			if a == b {
				continue;
			}
			for c in 0..=4 {
				if a == c || b == c {
					continue;
				}
				for d in 0..=4 {
					if a == d || b == d || c == d {
						continue;
					}
					for e in 0..=4 {
						if a == e || b == e || c == e || d == e {
							continue;
						}
						for phase_setting in [a, b, c, d, e] {
							output = run_program(program.clone(), phase_setting, input_signal);
							input_signal = output;
						}
						if output > highest_signal {
							highest_signal = output;
						}
						input_signal = 0;
					}
				}
			}
		}
	}
	highest_signal
}

pub fn run_program(mut program: Vec<isize>, phase_setting: isize, input_signal: isize) -> isize {
	let mut program_counter = 0;
	let mut input_count = 0;
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
				input_count += 1;
				let input_value = if input_count == 1 { phase_setting } else { input_signal };
				let output_pos = program[program_counter + 1];
				program[output_pos as usize] = input_value;
				program_counter += 2;
			}
			4 => {
				// Writes Output
				let output_value = program[program[program_counter + 1] as usize];
				return output_value;
			}
			5 => {
				let parameter_modes = instruction_header.to_digits();
				let operand_one: isize;
				if *parameter_modes.iter().nth_back(2).unwrap_or(&0) == 0 {
					operand_one = program[program[program_counter + 1] as usize];
				} else {
					operand_one = program[program_counter + 1];
				}
				if operand_one == 0 {
					program_counter += 3;
					continue;
				}
				let operand_two: isize;
				if *parameter_modes.iter().nth_back(3).unwrap_or(&0) == 0 {
					operand_two = program[program[program_counter + 2] as usize];
				} else {
					operand_two = program[program_counter + 2];
				}
				program_counter = operand_two as usize;
			}
			6 => {
				let parameter_modes = instruction_header.to_digits();
				let operand_one: isize;
				if *parameter_modes.iter().nth_back(2).unwrap_or(&0) == 0 {
					operand_one = program[program[program_counter + 1] as usize];
				} else {
					operand_one = program[program_counter + 1];
				}
				if operand_one != 0 {
					program_counter += 3;
					continue;
				}
				let operand_two: isize;
				if *parameter_modes.iter().nth_back(3).unwrap_or(&0) == 0 {
					operand_two = program[program[program_counter + 2] as usize];
				} else {
					operand_two = program[program_counter + 2];
				}
				program_counter = operand_two as usize;
			}
			7 => {
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
				let output_pos = program[program_counter + 3] as usize;
				if operand_one < operand_two {
					program[output_pos] = 1;
				} else {
					program[output_pos] = 0;
				}
				program_counter += 4;
			}
			8 => {
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
				let output_pos = program[program_counter + 3] as usize;
				if operand_one == operand_two {
					program[output_pos] = 1;
				} else {
					program[output_pos] = 0;
				}
				program_counter += 4;
			}
			99 => {
				// Halt
				println!("HLT");
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
