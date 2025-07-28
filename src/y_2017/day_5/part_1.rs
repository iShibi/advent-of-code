pub fn solve(input_text: &String) -> u32 {
	let mut jump_offsets =
		input_text.split_terminator('\n').filter_map(|line| line.parse::<i32>().ok()).collect::<Vec<_>>();
	let mut program_counter: i32 = 0;
	let mut steps = 0;
	while program_counter >= 0 && program_counter < (jump_offsets.len() as i32) {
		let jump_offset = jump_offsets[program_counter as usize];
		jump_offsets[program_counter as usize] += 1;
		program_counter += jump_offset;
		steps += 1;
	}
	steps
}
