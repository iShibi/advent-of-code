use std::collections::HashMap;

pub fn solve(input_text: &String) -> String {
	let mut correct_message = vec![];
	let messages = input_text
		.split_whitespace()
		.map(|str| str.split("").filter(|str| !str.is_empty()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let total_cols = messages[0].len();
	for col in 0..total_cols {
		let mut tracker: HashMap<&str, u32> = HashMap::new();
		for i in 0..messages.len() {
			let ch = messages[i][col];
			if let Some(count) = tracker.get_mut(ch) {
				*count += 1;
			} else {
				tracker.insert(ch, 1);
			}
		}
		let (ch, _) = tracker.iter().min_by_key(|entry| entry.1).unwrap();
		correct_message.push(*ch);
	}
	correct_message.join("")
}
