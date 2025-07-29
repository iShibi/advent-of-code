pub fn solve(input_text: &String) -> u32 {
	let stream = input_text.split_terminator('\n').collect::<String>().chars().collect::<Vec<char>>();
	let mut idx = 0;
	let mut currently_inside_garbage = false;
	let mut level = 0;
	let mut score = 0;
	while idx < stream.len() {
		let ch = stream[idx];
		match ch {
			'{' => {
				if !currently_inside_garbage {
					// Start Group
					level += 1;
				}
			}
			'}' => {
				if !currently_inside_garbage {
					// End Group
					score += level;
					level -= 1;
				}
			}
			'<' => {
				if !currently_inside_garbage {
					// Start Garbage
					currently_inside_garbage = true;
				}
			}
			'>' => {
				// End Garbage
				currently_inside_garbage = false;
			}
			'!' => {
				// Skip the next char
				idx += 2;
				continue;
			}
			_catch_all => {
				// Do nothing for other characters in stream
			}
		}
		idx += 1;
	}
	score
}
