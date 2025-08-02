use std::collections::HashMap;

pub fn solve(input_text: &String) -> u64 {
	let instructions = input_text.split_terminator('\n').collect::<Vec<_>>();
	let mut memory = HashMap::<String, String>::new();
	let mut mask = "";
	for instruction in instructions {
		let tokens = instruction
			.split_terminator('=')
			.flat_map(|str| {
				let parent_token = str.trim();
				if parent_token.contains("mem") {
					let child_tokens = parent_token
						.split(|ch| ch == '[' || ch == ']')
						.filter(|token| !token.is_empty())
						.collect::<Vec<_>>();
					child_tokens
				} else {
					vec![parent_token]
				}
			})
			.collect::<Vec<_>>();
		let opcode = tokens[0];
		match opcode {
			"mask" => {
				mask = tokens[1];
			}
			"mem" => {
				let addrs = tokens[1];
				let value = tokens[2].parse::<u64>().unwrap();
				let mut value_in_binary = format!("{:036b}", value).chars().rev().take(36).collect::<Vec<_>>();
				value_in_binary.reverse();
				for (bit_position, overwrite_value) in mask.char_indices() {
					match overwrite_value {
						'1' => value_in_binary[bit_position] = '1',
						'0' => value_in_binary[bit_position] = '0',
						_catch_all => {
							// Do nothing
						}
					}
				}
				let value_to_store = value_in_binary.iter().collect::<String>();
				memory.insert(addrs.to_string(), value_to_store);
			}
			_catch_all => panic!("Opcode '{_catch_all}' is not supported."),
		}
	}
	let mut sum = 0;
	for (_, binary_value) in &memory {
		let value = u64::from_str_radix(binary_value, 2).unwrap();
		sum += value;
	}
	sum
}
