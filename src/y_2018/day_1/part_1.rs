pub fn solve(input_text: &String) -> i32 {
	let frequency_changes =
		input_text.split_whitespace().filter_map(|line| line.parse::<i32>().ok()).collect::<Vec<_>>();
	frequency_changes.iter().sum()
}
