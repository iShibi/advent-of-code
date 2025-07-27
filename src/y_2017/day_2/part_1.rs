pub fn solve(input_text: &String) -> u32 {
	let mut checksum = 0;
	let mut spreadsheet = input_text
		.split_terminator('\n')
		.map(|line| line.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	for row in &mut spreadsheet {
		row.sort();
		checksum += row.last().unwrap() - row.first().unwrap();
	}
	checksum
}
