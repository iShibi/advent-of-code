use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let mut valid_passphrases = 0;
	let passphrases = input_text.split_terminator('\n').collect::<Vec<_>>();
	'outer: for passphrase in &passphrases {
		let mut tracker = HashMap::<String, u32>::new();
		for word in passphrase.split_whitespace() {
			let mut chars = word.split("").filter(|ch| !ch.is_empty()).collect::<Vec<_>>();
			chars.sort();
			let sorted_word = chars.join("");
			if tracker.contains_key(&sorted_word) {
				continue 'outer;
			} else {
				tracker.insert(sorted_word, 1);
			}
		}
		valid_passphrases += 1;
	}
	valid_passphrases
}
