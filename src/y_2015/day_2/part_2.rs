pub fn solve(input_text: &String) -> usize {
	let list_of_dimensions = input_text.split_whitespace().collect::<Vec<_>>();
	let mut ribbon_len = 0;
	for dimensions_str in list_of_dimensions {
		let mut dimensions = dimensions_str.split("x").filter_map(|d| d.parse::<usize>().ok()).collect::<Vec<_>>();
		let ribbon_for_bow = dimensions[0] * dimensions[1] * dimensions[2];
		dimensions.sort();
		let ribbon_for_wrapping = (2 * dimensions[0]) + (2 * dimensions[1]);
		ribbon_len += ribbon_for_bow + ribbon_for_wrapping;
	}
	ribbon_len
}
