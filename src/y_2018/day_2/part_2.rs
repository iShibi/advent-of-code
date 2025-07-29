pub fn solve(input_text: &String) -> String {
	let box_ids = input_text.split_terminator('\n').collect::<Vec<_>>();
	for left_idx in 0..(box_ids.len() - 1) {
		let left_box = box_ids[left_idx].chars().collect::<Vec<char>>();
		for right_idx in (left_idx + 1)..box_ids.len() {
			let right_box = box_ids[right_idx].chars().collect::<Vec<char>>();
			let mut different_char_count = 0;
			let mut different_char_idx = 0;
			for idx in 0..left_box.len() {
				if left_box[idx] != right_box[idx] {
					different_char_count += 1;
					different_char_idx = idx;
				}
			}
			if different_char_count == 1 {
				let mut res = left_box.clone();
				res.remove(different_char_idx);
				return res.iter().collect::<String>();
			}
		}
	}
	panic!("Never");
}
