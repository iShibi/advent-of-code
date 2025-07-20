use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let program =
		input_text.split(|c| c == '\n' || c == ',').filter_map(|c| c.parse::<isize>().ok()).collect::<Vec<isize>>();
	let mut memory = HashMap::<usize, isize>::new();
	for (adrs, num) in program.iter().enumerate() {
		memory.insert(adrs, num.clone());
	}
	let mut current_panel = (0, 0);
	let mut current_orientation = Orientation::Up;
	let mut ship_hull = HashMap::<(isize, isize), Color>::new();
	let mut program_counter = 0;
	let mut relative_base_offset = 0;
	loop {
		let input_value = *(ship_hull.get(&current_panel).unwrap_or(&Color::Black));
		let (output, memory_, program_counter_, relative_base_offset_, has_halted) = run_program(
			memory.clone(),
			program_counter,
			relative_base_offset,
			input_value.into(),
		);
		if has_halted {
			break;
		}
		let (paint_color, direction) = (output[0], output[1]);
		ship_hull.insert(current_panel, paint_color.into());
		current_orientation = current_orientation.update_orientation(direction);
		match current_orientation {
			Orientation::Up => current_panel = (current_panel.0, current_panel.1 + 1),
			Orientation::Left => current_panel = (current_panel.0 - 1, current_panel.1),
			Orientation::Down => current_panel = (current_panel.0, current_panel.1 - 1),
			Orientation::Right => current_panel = (current_panel.0 + 1, current_panel.1),
		}
		(memory, program_counter, relative_base_offset) = (memory_, program_counter_, relative_base_offset_);
	}
	ship_hull.iter().count()
}

pub fn run_program(
	mut memory: HashMap<usize, isize>,
	mut program_counter: usize,
	mut relative_base_offset: isize,
	input_value: isize,
) -> (Vec<isize>, HashMap<usize, isize>, usize, isize, bool) {
	let mut output = vec![];
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
				let store_at = get_operand(
					&memory,
					program_counter + 1,
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
				output.push(output_value);
				program_counter += 2;
				if output.len() == 2 {
					// println!("{:#?}", output);
					return (output, memory.clone(), program_counter, relative_base_offset, false);
				}
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
				return (output, memory.clone(), program_counter, relative_base_offset, true);
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

#[derive(Debug, Clone, Copy)]
enum Color {
	White,
	Black,
}

impl From<isize> for Color {
	fn from(value: isize) -> Self {
		match value {
			0 => Self::Black,
			1 => Self::White,
			_catch_all => panic!("Only Black (0) and White (1) color are supported. Found {_catch_all}."),
		}
	}
}

impl Into<isize> for Color {
	fn into(self) -> isize {
		match self {
			Self::Black => 0,
			Self::White => 1,
		}
	}
}

enum Orientation {
	Up,
	Down,
	Left,
	Right,
}

impl Orientation {
	fn update_orientation(&self, direction: isize) -> Self {
		match self {
			Self::Up => match direction {
				0 => Self::Left,
				1 => Self::Right,
				_catch_all => panic!("Only Left (0) and Right (1) direction are supported. Found {_catch_all}."),
			},
			Self::Left => match direction {
				0 => Self::Down,
				1 => Self::Up,
				_catch_all => panic!("Only Left (0) and Right (1) direction are supported. Found {_catch_all}."),
			},
			Self::Down => match direction {
				0 => Self::Right,
				1 => Self::Left,
				_catch_all => panic!("Only Left (0) and Right (1) direction are supported. Found {_catch_all}."),
			},
			Self::Right => match direction {
				0 => Self::Up,
				1 => Self::Down,
				_catch_all => panic!("Only Left (0) and Right (1) direction are supported. Found {_catch_all}."),
			},
		}
	}
}
