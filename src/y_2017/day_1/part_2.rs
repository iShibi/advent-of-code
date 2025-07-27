pub fn solve(input_text: &String) -> u32 {
	let mut sum = 0;
	let digit_sequence = input_text.split_whitespace().collect::<String>().chars().collect::<Vec<_>>();
	for idx in 0..digit_sequence.len() {
		let ch = digit_sequence[idx];
		let next_idx = (idx + (digit_sequence.len() / 2)) % digit_sequence.len();
		if ch == digit_sequence[next_idx] {
			sum += ch.to_digit(10).unwrap();
		}
	}
	sum
}
