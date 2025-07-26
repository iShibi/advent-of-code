use std::collections::HashMap;

pub fn solve(input_string: &String) -> i32 {
	let instructions = input_string.split_terminator('\n').collect::<Vec<_>>();
	let mut registers = HashMap::<&str, i32>::from_iter([("a", 0), ("b", 0), ("c", 0), ("d", 0)]);
	let mut program_counter: i32 = 0;
	loop {
		if program_counter < 0 {
			break;
		}
		let tokens = instructions[program_counter as usize].split_whitespace().collect::<Vec<_>>();
		let opcode = tokens[0];
		match opcode {
			"cpy" => {
				let register_or_value = tokens[1];
				let value = register_or_value
					.parse::<i32>()
					.ok()
					.unwrap_or_else(|| registers.get(register_or_value).copied().unwrap());
				let store_in_register = tokens[2];
				registers.insert(store_in_register, value);
				program_counter += 1;
			}
			"inc" => {
				let register = tokens[1];
				let current_value = registers.get_mut(register).unwrap();
				*current_value += 1;
				program_counter += 1;
			}
			"dec" => {
				let register = tokens[1];
				let current_value = registers.get_mut(register).unwrap();
				*current_value -= 1;
				program_counter += 1;
			}
			"jnz" => {
				let register_or_value = tokens[1];
				let value = register_or_value
					.parse::<i32>()
					.ok()
					.unwrap_or_else(|| registers.get(register_or_value).copied().unwrap());
				// let register_or_value = tokens[1];
				let register_or_jump_count = tokens[2];
				let jump_count = register_or_jump_count
					.parse::<i32>()
					.ok()
					.unwrap_or_else(|| registers.get(register_or_jump_count).copied().unwrap());
				if value != 0 {
					program_counter += jump_count;
				} else {
					program_counter += 1;
				}
			}
			_catch_all => panic!("The opcode '{_catch_all}' is not supported."),
		}
		if program_counter >= instructions.len() as i32 {
			break;
		}
	}
	*registers.get("a").unwrap()
}
