use std::fs;

pub fn solve() -> usize {
	let input_text = fs::read_to_string("src/y_2019/day_2/input.txt").unwrap();
	let mut integers = input_text
		.split(|c| c == ',' || c == '\n')
		.filter_map(|c| usize::from_str_radix(c, 10).ok())
		.collect::<Vec<_>>();
	integers[1] = 12;
	integers[2] = 2;
	let mut i = 0;
	loop {
		let opcode = integers[i];
		match opcode {
			1 => {
				// Add
				let operand_one = integers[integers[i + 1]];
				let operand_two = integers[integers[i + 2]];
				let sum = operand_one + operand_two;
				let output_pos = integers[i + 3];
				integers[output_pos] = sum;
			}
			2 => {
				// Multiply
				let operand_one = integers[integers[i + 1]];
				let operand_two = integers[integers[i + 2]];
				let product = operand_one * operand_two;
				let output_pos = integers[i + 3];
				integers[output_pos] = product;
			}
			99 => {
				// Halt
				break;
			}
			_ => {
				// Error
			}
		}
		i += 4;
	}
	integers[0]
}
