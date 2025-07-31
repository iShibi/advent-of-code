use std::collections::HashMap;

pub fn solve(input_text: &String) -> i32 {
	let instructions = input_text
		.split_terminator('\n')
		.map(|line| line.split_whitespace().collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut accumulator = 0;
	let mut program_counter: i32 = 0;
	let mut tracker = HashMap::<i32, bool>::new();
	loop {
		if tracker.contains_key(&program_counter) {
			return accumulator;
		} else {
			tracker.insert(program_counter, true);
		}
		let instruction = &instructions[program_counter as usize];
		let (opcode, operand) = (instruction[0], instruction[1].parse::<i32>().unwrap());
		match opcode {
			"nop" => {
				program_counter += 1;
			}
			"acc" => {
				accumulator += operand;
				program_counter += 1;
			}
			"jmp" => {
				program_counter += operand;
			}
			_catch_all => panic!("The opcode '{_catch_all}' is not supported."),
		}
	}
}
