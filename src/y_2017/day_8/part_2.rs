use std::collections::HashMap;

pub fn solve(input_text: &String) -> i32 {
	let instructions = input_text.split_terminator('\n').collect::<Vec<_>>();
	let mut registers = HashMap::<&str, i32>::new();
	let mut highest_value = 0;
	for instruction in instructions {
		let tokens = instruction.split_whitespace().collect::<Vec<_>>();
		let register = tokens[0];
		let operation = tokens[1];
		let amount = tokens[2].parse::<i32>().unwrap();
		let lhs = registers.get(tokens[4]).unwrap_or(&0);
		let condition = tokens[5];
		let rhs = tokens[6].parse::<i32>().unwrap();
		match condition {
			">" => {
				if *lhs > rhs {
					perform_operation(register, operation, amount, &mut registers, &mut highest_value);
				}
			}
			"<" => {
				if *lhs < rhs {
					perform_operation(register, operation, amount, &mut registers, &mut highest_value);
				}
			}
			">=" => {
				if *lhs >= rhs {
					perform_operation(register, operation, amount, &mut registers, &mut highest_value);
				}
			}
			"==" => {
				if *lhs == rhs {
					perform_operation(register, operation, amount, &mut registers, &mut highest_value);
				}
			}
			"<=" => {
				if *lhs <= rhs {
					perform_operation(register, operation, amount, &mut registers, &mut highest_value);
				}
			}
			"!=" => {
				if *lhs != rhs {
					perform_operation(register, operation, amount, &mut registers, &mut highest_value);
				}
			}
			_catch_all => panic!("Condition '{_catch_all}' is not supported."),
		}
	}
	highest_value
}

fn perform_operation<'a>(
	register: &'a str,
	operation: &str,
	amount: i32,
	registers: &mut HashMap<&'a str, i32>,
	highest_value: &mut i32,
) {
	match operation {
		"inc" => {
			if let Some(value) = registers.get_mut(register) {
				let result = *value + amount;
				*value = result;
				if result > *highest_value {
					*highest_value = result;
				}
			} else {
				registers.insert(register, amount);
				if amount > *highest_value {
					*highest_value = amount;
				}
			}
		}
		"dec" => {
			if let Some(value) = registers.get_mut(register) {
				let result = *value - amount;
				*value = result;
				if result > *highest_value {
					*highest_value = result;
				}
			} else {
				let result = 0 - amount;
				registers.insert(register, result);
				if result > *highest_value {
					*highest_value = result;
				}
			}
		}
		_catch_all => panic!("Operation '{_catch_all}' is not supported."),
	}
}
