pub fn solve(input_text: &String) -> u32 {
	let stream = input_text.split_terminator('\n').collect::<String>().chars().collect::<Vec<char>>();
	let mut idx = 0;
	let mut currently_inside_garbage = false;
	let mut non_cancelled_garbage_chars = 0;
	while idx < stream.len() {
		let ch = stream[idx];
		match ch {
			'<' => {
				if !currently_inside_garbage {
					// Start Garbage
					currently_inside_garbage = true;
				} else {
					non_cancelled_garbage_chars += 1;
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
				if currently_inside_garbage {
					non_cancelled_garbage_chars += 1;
				}
			}
		}
		idx += 1;
	}
	non_cancelled_garbage_chars
}
