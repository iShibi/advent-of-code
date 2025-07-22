use std::collections::HashMap;

pub fn solve(input_text: &String) -> isize {
	let program = input_text
		.split(|c| c == '\n' || c == ',')
		.filter_map(|c| c.parse::<isize>().ok())
		.collect::<Vec<isize>>();
	let mut highest_signal = 0;
	for a in 5..=9 {
		for b in 5..=9 {
			if a == b {
				continue;
			}
			for c in 5..=9 {
				if a == c || b == c {
					continue;
				}
				for d in 5..=9 {
					if a == d || b == d || c == d {
						continue;
					}
					for e in 5..=9 {
						if a == e || b == e || c == e || d == e {
							continue;
						}
						let mut program_data = program.clone();
						let mut input_signal = 0;
						let output: isize;
						let mut program_counter = 0;
						let mut input_count = 0;
						let mut program_metadata: HashMap<isize, (Vec<isize>, usize, i32)> = HashMap::new();
						'feedback: loop {
							for phase_setting in [a, b, c, d, e] {
								if let Some((running_program_, program_counter_, input_count_)) =
									program_metadata.get(&phase_setting)
								{
									program_data = running_program_.clone();
									program_counter = program_counter_.clone();
									input_count = input_count_.clone();
								}
								let (output_, has_halted_, program_counter_, input_count_, running_program_) =
									run_program(
										program_data.clone(),
										phase_setting,
										input_signal,
										program_counter,
										input_count,
									);
								if has_halted_ {
									output = input_signal;
									break 'feedback;
								}
								input_signal = output_;
								program_metadata.insert(
									phase_setting,
									(running_program_.clone(), program_counter_, input_count_),
								);
							}
						}
						if output > highest_signal {
							highest_signal = output;
						}
					}
				}
			}
		}
	}
	highest_signal
}

pub fn run_program(
	mut program: Vec<isize>,
	phase_setting: isize,
	input_signal: isize,
	mut program_counter: usize,
	mut input_count: i32,
) -> (isize, bool, usize, i32, Vec<isize>) {
	// let mut program_counter = 0;
	// let mut input_count = 0;
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
				program_counter += 2;
				return (output_value, false, program_counter, input_count, program.clone());
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
				program_counter += 1;
				return (0, true, program_counter, input_count, program.clone());
			}
			_ => {
				// Error
				panic!();
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
