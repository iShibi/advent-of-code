pub fn solve(input_text: &String) -> u32 {
	let entries = input_text.split_terminator('\n').filter_map(|line| line.parse::<u32>().ok()).collect::<Vec<_>>();
	for left_idx in 0..entries.len() - 2 {
		let left_entry = entries[left_idx];
		for middle_idx in left_idx + 1..entries.len() - 1 {
			let middle_entry = entries[middle_idx];
			for right_idx in middle_idx + 1..entries.len() {
				let right_entry = entries[right_idx];
				if left_entry + middle_entry + right_entry == 2020 {
					return left_entry * middle_entry * right_entry;
				}
			}
		}
	}
	panic!("Never");
}
