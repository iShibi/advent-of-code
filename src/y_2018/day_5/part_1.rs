pub fn solve(input_text: &String) -> usize {
	let mut polymer = input_text.split_terminator('\n').collect::<String>().chars().collect::<Vec<char>>();
	'outer: loop {
		let mut idx = 0;
		let mut found = false;
		let buffer = polymer.clone();
		'inner: while idx < buffer.len() - 1 {
			let left_ch = buffer[idx];
			let right_ch = buffer[idx + 1];
			if left_ch != right_ch && left_ch.to_ascii_lowercase() == right_ch.to_ascii_lowercase() {
				found = true;
				break 'inner;
			}
			idx += 1;
		}
		if found {
			polymer.remove(idx);
			polymer.remove(idx);
		} else {
			break 'outer;
		}
	}
	polymer.len()
}
