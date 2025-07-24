use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let rooms = input_text.split_whitespace().collect::<Vec<_>>();
	for room in &rooms {
		let data = room
			.split(|ch| ch == '-' || ch == '[' || ch == ']')
			.filter(|str| !str.is_empty())
			.collect::<Vec<_>>();
		let encrypted_name = data[..data.len() - 2].iter().map(|item| item.to_string()).collect::<Vec<_>>().join("");
		let sector_id = data.iter().nth_back(1).unwrap().parse::<u32>().unwrap();
		let checksum = data.last().unwrap();
		let encrypted_name_with_dash =
			data[..data.len() - 2].iter().map(|item| item.to_string()).collect::<Vec<_>>().join("-");
		if five_most_common_letters_in(&encrypted_name) == checksum.to_string() {
			let mut name = String::new();
			let offset = sector_id % 26;
			for ch in encrypted_name_with_dash.chars() {
				if ch == '-' {
					name.push(' ');
					continue;
				}
				let ascii_value = (ch as u8) + (offset as u8);
				if ascii_value <= 122 {
					let offset_ch = ascii_value as char;
					name.push(offset_ch);
				} else {
					let offset_ch = ((ascii_value + 96) % 122) as char;
					name.push(offset_ch);
				}
			}
			if name.contains("northpole object storage") {
				return sector_id;
			}
		}
	}
	panic!("Never");
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
