use std::collections::HashMap;

pub fn solve(input_text: &String) -> i32 {
	let frequency_changes =
		input_text.split_whitespace().filter_map(|line| line.parse::<i32>().ok()).collect::<Vec<_>>();
	let mut frequency_tracker = HashMap::<i32, bool>::new();
	let mut frequency = 0;
	loop {
		for change in &frequency_changes {
			frequency += change;
			if frequency_tracker.contains_key(&frequency) {
				return frequency;
			} else {
				frequency_tracker.insert(frequency, true);
			}
		}
	}
}
