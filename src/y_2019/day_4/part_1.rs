pub fn solve() -> u32 {
	let mut password_count = 0;
	for password in 158126..=624574 {
		let digits = to_digits(password);
		let mut i = 0;
		let mut j = i + 1;
		let (mut is_decreasing, mut has_equal_adjacent_values) = (true, false);
		loop {
			if j == digits.len() {
				break;
			}
			if digits[i] > digits[j] {
				is_decreasing = false;
				break;
			}
			if digits[i] == digits[j] && !has_equal_adjacent_values {
				has_equal_adjacent_values = true;
			}
			i += 1;
			j += 1;
		}
		if is_decreasing && has_equal_adjacent_values {
			password_count += 1;
		}
	}
	password_count
}

fn to_digits(mut num: i32) -> Vec<i32> {
	let mut digits = vec![];
	loop {
		let digit = num % 10;
		digits.push(digit);
		num = num / 10;
		if num == 0 {
			break;
		}
	}
	digits.reverse();
	digits
}
