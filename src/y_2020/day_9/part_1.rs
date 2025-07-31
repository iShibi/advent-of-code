pub fn solve(input_text: &String) -> usize {
	let numbers = input_text.split_whitespace().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<_>>();
	let preamble_size = 25;
	for idx in preamble_size..numbers.len() {
		let num = numbers[idx];
		let start_idx = idx - preamble_size;
		if !is_sum_of_any_two(num, &numbers[start_idx..idx]) {
			return num;
		}
	}
	panic!("Never")
}

fn is_sum_of_any_two(num: usize, prev_numbers: &[usize]) -> bool {
	for left_idx in 0..(prev_numbers.len() - 1) {
		let left_num = prev_numbers[left_idx];
		for right_idx in (left_idx + 1)..prev_numbers.len() {
			let right_num = prev_numbers[right_idx];
			if left_num + right_num == num {
				return true;
			}
		}
	}
	false
}
