use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let list_of_strings = input_text.split_whitespace().collect::<Vec<_>>();
	let mut nice_strings_count = 0;
	for string in list_of_strings {
		let (mut condition_one, mut condition_two) = (false, false);
		let mut tracker = HashMap::new();
		let mut prev_pair = "".as_ref();
		for i in 0..string.len() - 1 {
			let pair = &string[i..=i + 1];
			if prev_pair == pair {
				prev_pair = "";
				continue;
			}
			if tracker.contains_key(pair) {
				condition_one = true;
				break;
			} else {
				tracker.insert(pair, 1);
			}
			prev_pair = pair;
		}
		for i in 0..string.len() - 2 {
			let triplet = &string[i..=i + 2].chars().collect::<Vec<_>>();
			if triplet[0] == triplet[2] && triplet[0] != triplet[1] {
				condition_two = true;
				break;
			}
		}
		if condition_one && condition_two {
			nice_strings_count += 1;
		}
	}
	nice_strings_count
}
