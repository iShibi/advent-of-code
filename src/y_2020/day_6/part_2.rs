use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let groups = input_text
		.split("\n\n")
		.map(|group| group.split_terminator('\n').collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut sum = 0;
	for group in &groups {
		let mut tracker = HashMap::<&str, usize>::new();
		for person in group {
			let questions = person.split("").filter(|str| !str.is_empty());
			for question in questions {
				if let Some(count) = tracker.get_mut(question) {
					*count += 1;
				} else {
					tracker.insert(question, 1);
				}
			}
		}
		for (_, count) in tracker {
			if count == group.len() {
				sum += 1;
			}
		}
	}
	sum
}
