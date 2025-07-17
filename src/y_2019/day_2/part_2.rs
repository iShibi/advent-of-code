pub fn solve(input_text: &String) -> usize {
	let integers = input_text
		.split(|c| c == ',' || c == '\n')
		.filter_map(|c| usize::from_str_radix(c, 10).ok())
		.collect::<Vec<_>>();
	let mut noun = 0;
	let mut verb = 0;
	'outer: for i in 0..=99 {
		for j in 0..=99 {
			let output = run_program(integers.clone(), i, j);
			if output == 19690720 {
				noun = i;
				verb = j;
				break 'outer;
			};
		}
	}
	100 * noun + verb
}

pub fn run_program(mut program: Vec<usize>, noun: usize, verb: usize) -> usize {
	program[1] = noun;
	program[2] = verb;
	let mut i = 0;
	loop {
		let opcode = program[i];
		match opcode {
			1 => {
				// Add
				let operand_one = program[program[i + 1]];
				let operand_two = program[program[i + 2]];
				let sum = operand_one + operand_two;
				let output_pos = program[i + 3];
				program[output_pos] = sum;
			}
			2 => {
				// Multiply
				let operand_one = program[program[i + 1]];
				let operand_two = program[program[i + 2]];
				let product = operand_one * operand_two;
				let output_pos = program[i + 3];
				program[output_pos] = product;
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
	program[0]
}
