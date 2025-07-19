pub fn solve(input_text: &String) -> i32 {
	let mut number: i32 = 0;
	let mut numbers: Vec<i32> = vec![];
	let mut num_start = false;
	let doc = input_text.chars().collect::<Vec<_>>();
	for (idx, ch) in doc.iter().enumerate() {
		if ch.is_digit(10) {
			number = (number * 10) + (ch.to_digit(10).unwrap() as i32);
			num_start = true;
		} else if num_start {
			let digit_count = number.to_string().len();
			let sign_pos = idx - digit_count - 1;
			if doc[sign_pos] == '-' {
				number = number - (2 * number);
			}
			numbers.push(number);
			number = 0;
			num_start = false;
		}
	}
	numbers.iter().sum()
}
