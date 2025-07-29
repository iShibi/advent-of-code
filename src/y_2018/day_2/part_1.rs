use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let box_ids = input_text.split_terminator('\n').collect::<Vec<_>>();
	let mut twos = 0;
	let mut threes = 0;
	for id in box_ids {
		let mut char_tracker = HashMap::<char, u32>::new();
		let chars = id.chars();
		for ch in chars {
			if let Some(count) = char_tracker.get_mut(&ch) {
				*count += 1;
			} else {
				char_tracker.insert(ch, 1);
			}
		}
		if let Some(_) = char_tracker.iter().find(|(_, v)| **v == 2) {
			twos += 1;
		}
		if let Some(_) = char_tracker.iter().find(|(_, v)| **v == 3) {
			threes += 1;
		}
	}
	twos * threes
}
