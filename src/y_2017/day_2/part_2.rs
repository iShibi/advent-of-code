pub fn solve(input_text: &String) -> u32 {
	let mut sum = 0;
	let mut spreadsheet = input_text
		.split_terminator('\n')
		.map(|line| line.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	for row in &mut spreadsheet {
		row.sort();
		for dividend_idx in (1..row.len()).rev() {
			for divisor_idx in (0..dividend_idx).rev() {
				let dividend = row[dividend_idx];
				let divisor = row[divisor_idx];
				if dividend % divisor == 0 {
					sum += row[dividend_idx] / row[divisor_idx];
				}
			}
		}
	}
	sum
}
