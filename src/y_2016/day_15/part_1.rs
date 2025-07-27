pub fn solve(input_text: &String) -> u32 {
	let discs = input_text
		.split_terminator('\n')
		.map(|line| {
			line.replace(".", "")
				.split_whitespace()
				.filter_map(|word| word.parse::<u32>().ok())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	let mut button_press_time = 0;
	'outer: loop {
		let mut passed_through_count = 0;
		let mut time = button_press_time + 1;
		'inner: for disc in &discs {
			let (total_positions, start_position) = (disc[0], disc[1]);
			let current_postion = (start_position + time) % total_positions;
			if current_postion == 0 {
				passed_through_count += 1;
				time += 1;
			} else {
				break 'inner;
			}
		}
		if passed_through_count == discs.len() {
			break 'outer;
		}
		button_press_time += 1;
	}
	button_press_time
}
