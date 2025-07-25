pub fn solve(input_text: &String) -> usize {
	let file = input_text.chars().collect::<Vec<char>>();
	let mut inside_marker = false;
	let mut marker = String::new();
	let mut decompressed_file = String::new();
	let mut idx = 0;
	loop {
		if idx >= input_text.len() {
			break;
		}
		let ch = file[idx];
		match ch {
			maybe_whitespace if maybe_whitespace.is_whitespace() => {
				idx += 1;
				continue;
			}
			'(' => inside_marker = true,
			')' => {
				inside_marker = false;
				let char_count = marker.split("x").nth(0).unwrap().parse::<usize>().unwrap();
				let repeat_count = marker.split("x").nth(1).unwrap().parse::<usize>().unwrap();
				let sequence_to_repeat = &file[(idx + 1)..=(idx + char_count)].iter().collect::<String>();
				for _ in 1..=repeat_count {
					decompressed_file.push_str(sequence_to_repeat);
				}
				marker.clear();
				idx += char_count + 1;
				continue;
			}
			_ => {
				if inside_marker {
					marker.push(ch);
				} else {
					decompressed_file.push(ch);
				}
			}
		}
		idx += 1;
	}
	decompressed_file.len()
}
