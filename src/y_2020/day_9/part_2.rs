pub fn solve(input_text: &String) -> usize {
	let numbers = input_text.split_whitespace().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<_>>();
	let preamble_size = 25;
	let mut invalid_number = 0;
	for idx in preamble_size..numbers.len() {
		let num = numbers[idx];
		let start_idx = idx - preamble_size;
		if !is_sum_of_any_two(num, &numbers[start_idx..idx]) {
			invalid_number = num;
			break;
		}
	}
	for initial_idx in 0..numbers.len() - 1 {
		let initial_num = numbers[initial_idx];
		let mut sum = initial_num;
		let mut arr = vec![initial_num];
		let mut idx = initial_idx + 1;
		'inner: loop {
			let num = numbers[idx];
			arr.push(num);
			sum += num;
			if sum == invalid_number {
				arr.sort();
				return arr.first().unwrap() + arr.last().unwrap();
			} else if sum > invalid_number {
				break 'inner;
			}
			idx += 1;
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
