use std::collections::HashMap;

pub fn solve(input_text: &String) -> i32 {
	let instructions = input_text
		.split_terminator('\n')
		.map(|line| line.split_whitespace().collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let addrs_of_nops_and_jpms = instructions
		.iter()
		.enumerate()
		.filter_map(|(idx, ins)| (ins.contains(&"nop") || ins.contains(&"jmp")).then_some(idx))
		.collect::<Vec<_>>();
	let mut idx = 0;
	let mut accumulator = 0;
	let mut program_terminated = false;
	while !program_terminated {
		(accumulator, program_terminated) = run_program(&instructions, addrs_of_nops_and_jpms[idx]);
		idx += 1;
	}
	accumulator
}

fn run_program(instructions: &Vec<Vec<&str>>, addrs_of_opcode_to_switch: usize) -> (i32, bool) {
	let mut accumulator = 0;
	let mut program_counter: i32 = 0;
	let mut tracker = HashMap::<i32, bool>::new();
	'program: loop {
		if tracker.contains_key(&program_counter) {
			return (accumulator, false);
		} else {
			tracker.insert(program_counter, true);
		}
		let instruction = &instructions[program_counter as usize];
		let (opcode, operand) = (instruction[0], instruction[1].parse::<i32>().unwrap());
		match opcode {
			"nop" => {
				if program_counter == addrs_of_opcode_to_switch as i32 {
					// act as jmp
					program_counter += operand;
				} else {
					program_counter += 1;
				}
			}
			"acc" => {
				accumulator += operand;
				program_counter += 1;
			}
			"jmp" => {
				if program_counter == addrs_of_opcode_to_switch as i32 {
					// act as nop
					program_counter += 1;
				} else {
					program_counter += operand;
				}
			}
			_catch_all => panic!("The opcode '{_catch_all}' is not supported."),
		}
		if program_counter >= instructions.len() as i32 {
			break 'program;
		}
	}
	return (accumulator, true);
}
