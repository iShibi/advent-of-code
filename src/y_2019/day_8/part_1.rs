pub fn solve(input_text: &String) -> u32 {
	let digits = input_text.split("").filter_map(|c| c.parse::<u8>().ok()).collect::<Vec<u8>>();
	let (image_width, image_height) = (25, 6);
	let mut digit_count = 0;
	let (mut count_zero, mut count_one, mut count_two) = (0, 0, 0);
	let mut layers = vec![];
	for digit in digits {
		digit_count += 1;
		match digit {
			0 => count_zero += 1,
			1 => count_one += 1,
			2 => count_two += 1,
			_ => {
				// Do nothing
			}
		}
		if digit_count == (image_width * image_height) {
			layers.push((count_zero, count_one, count_two));
			(count_zero, count_one, count_two, digit_count) = (0, 0, 0, 0);
		}
	}
	layers.sort_by(|a, b| a.0.cmp(&b.0));
	let (_, count_one, count_two) = layers[0];
	count_one * count_two
}
