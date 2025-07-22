use std::collections::HashMap;

pub fn solve(input_text: &String) -> String {
	let base_pattern = vec![0, 1, 0, -1];
	let mut signal = input_text.split_whitespace().collect::<String>();
	let mut pattern_map = HashMap::<usize, Vec<i32>>::new();
	let signal_size = signal.len();
	let phases = 100;
	for _ in 1..=phases {
		let mut next_signal = String::with_capacity(signal_size);
		for idx_a in 0..signal_size {
			let pattern_for_idx_a = pattern_map.get(&idx_a).cloned().unwrap_or_else(|| {
				let repeat_count = idx_a + 1;
				let new_pattern = base_pattern
					.iter()
					.flat_map(|item| std::iter::repeat(item).take(repeat_count))
					.cloned()
					.collect::<Vec<i32>>();
				pattern_map.insert(idx_a, new_pattern.clone());
				new_pattern
			});
			let mut sum = 0;
			for (idx_b, ch) in signal.chars().enumerate() {
				sum += (ch.to_digit(10).unwrap() as i32) * get_pattern_value(idx_b, &pattern_for_idx_a);
			}
			let digit = sum.abs() % 10;
			next_signal.push((b'0' + digit as u8) as char);
		}
		signal = next_signal;
	}
	signal.chars().take(8).collect::<String>()
}

fn get_pattern_value(idx_b: usize, pattern_for_idx_a: &Vec<i32>) -> i32 {
	let count_of_values_to_skip = 1;
	let idx_of_value = idx_b + count_of_values_to_skip;
	let wrap_limit = pattern_for_idx_a.len();
	pattern_for_idx_a[idx_of_value % wrap_limit]
}
