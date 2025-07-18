pub fn solve(input_text: &String) -> usize {
	let list_of_dimensions = input_text.split_whitespace().collect::<Vec<_>>();
	let mut wrapping_paper_area = 0;
	for dimensions in list_of_dimensions {
		if let [l, w, h] = dimensions.split("x").filter_map(|d| d.parse::<usize>().ok()).collect::<Vec<_>>()[..] {
			let surface_area = (2 * l * w) + (2 * w * h) + (2 * h * l);
			let slack = (l * w * h) / [l, w, h].iter().max().unwrap();
			wrapping_paper_area += surface_area + slack;
		}
	}
	wrapping_paper_area
}
