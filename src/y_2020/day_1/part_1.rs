pub fn solve(input_text: &String) -> u32 {
	let entries = input_text.split_terminator('\n').filter_map(|line| line.parse::<u32>().ok()).collect::<Vec<_>>();
	for left_idx in 0..entries.len() - 1 {
		let left_entry = entries[left_idx];
		for right_idx in left_idx + 1..entries.len() {
			let right_entry = entries[right_idx];
			if left_entry + right_entry == 2020 {
				return left_entry * right_entry;
			}
		}
	}
	panic!("Never");
}
