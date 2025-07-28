use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let mut valid_passphrases = 0;
	let passphrases = input_text.split_terminator('\n').collect::<Vec<_>>();
	'outer: for passphrase in &passphrases {
		let mut tracker = HashMap::<&str, u32>::new();
		for word in passphrase.split_whitespace() {
			if tracker.contains_key(word) {
				continue 'outer;
			} else {
				tracker.insert(word, 1);
			}
		}
		valid_passphrases += 1;
	}
	valid_passphrases
}
