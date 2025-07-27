pub fn solve() -> String {
	let disk_size: usize = 272;
	let initial_state = "01000100010010111";
	let mut data = format!("{initial_state}");
	while data.len() < disk_size {
		let mut rev_data = String::new();
		for ch in data.chars().rev() {
			match ch {
				'0' => rev_data.push('1'),
				'1' => rev_data.push('0'),
				_catch_all => panic!("Only '0' and '1' are supported. Found '{_catch_all}'."),
			}
		}
		data = format!("{data}0{rev_data}");
	}
	let required_data = data.get(0..disk_size).unwrap();
	let mut maybe_checksum = required_data.to_string();
	let mut checksum = String::new();
	while checksum.len() % 2 == 0 {
		checksum = "".to_string();
		for idx in (0..=(maybe_checksum.len() - 2)).step_by(2) {
			let pair = maybe_checksum.get(idx..=(idx + 1)).unwrap().chars().collect::<Vec<_>>();
			let (char_one, char_two) = (pair[0], pair[1]);
			if char_one == char_two {
				checksum += "1";
			} else {
				checksum += "0";
			}
		}
		maybe_checksum = checksum.clone();
	}
	checksum
}
