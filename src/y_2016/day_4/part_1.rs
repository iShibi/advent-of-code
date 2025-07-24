use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let mut sum_of_sector_ids = 0;
	let rooms = input_text.split_whitespace().collect::<Vec<_>>();
	for room in &rooms {
		let data = room
			.split(|ch| ch == '-' || ch == '[' || ch == ']')
			.filter(|str| !str.is_empty())
			.collect::<Vec<_>>();
		let encrypted_name = data[..data.len() - 2].iter().map(|item| item.to_string()).collect::<Vec<_>>().join("");
		let sector_id = data.iter().nth_back(1).unwrap().parse::<u32>().unwrap();
		let checksum = data.last().unwrap();
		if five_most_common_letters_in(&encrypted_name) == checksum.to_string() {
			sum_of_sector_ids += sector_id;
		}
	}
	sum_of_sector_ids
}

fn five_most_common_letters_in(string: &String) -> String {
	let mut tracker = HashMap::<char, usize>::new();
	for ch in string.chars() {
		if let Some(count) = tracker.get_mut(&ch) {
			*count += 1;
		} else {
			tracker.insert(ch, 1);
		}
	}
	let mut buckets = vec![vec![]; string.len()];
	for (char, count) in &tracker {
		buckets.get_mut(string.len() - *count).unwrap().push(*char);
		buckets.get_mut(string.len() - *count).unwrap().sort();
	}
	let mut five_most_common_letter = String::new();
	'outer: for bucket in buckets {
		for ch in bucket {
			five_most_common_letter.push(ch);
			if five_most_common_letter.len() == 5 {
				break 'outer;
			}
		}
	}
	five_most_common_letter
}
