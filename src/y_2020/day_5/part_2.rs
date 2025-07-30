pub fn solve(input_text: &String) -> u32 {
	let boarding_passes = input_text.split_terminator('\n').collect::<Vec<_>>();
	let mut seat_ids = vec![];
	for pass in boarding_passes {
		let seat_id = get_seat_id(pass);
		seat_ids.push(seat_id);
	}
	seat_ids.sort();
	for idx in 1..seat_ids.len() - 1 {
		if seat_ids[idx] + 1 != seat_ids[idx + 1] {
			return seat_ids[idx] + 1;
		}
	}
	panic!("Never");
}

fn get_seat_id(pass: &str) -> u32 {
	let (mut lower_row, mut upper_row, mut row) = (0, 127, 0);
	for (idx, ch) in pass.char_indices() {
		if idx > 6 {
			break;
		}
		match ch {
			'F' => {
				upper_row = ((lower_row + upper_row) as f32 / 2 as f32).floor() as u32;
				row = lower_row;
			}
			'B' => {
				lower_row = ((lower_row + upper_row) as f32 / 2 as f32).ceil() as u32;
				row = upper_row;
			}
			_catch_all => panic!("'{_catch_all}' is not a supported row instruction."),
		}
	}
	let (mut lower_col, mut upper_col, mut col) = (0, 7, 0);
	for (_, ch) in pass.char_indices().skip(7) {
		match ch {
			'L' => {
				upper_col = ((lower_col + upper_col) as f32 / 2 as f32).floor() as u32;
				col = lower_col;
			}
			'R' => {
				lower_col = ((lower_col + upper_col) as f32 / 2 as f32).ceil() as u32;
				col = upper_col;
			}
			_catch_all => panic!("'{_catch_all}' is not a supported col instruction."),
		}
	}
	(row * 8) + col
}
