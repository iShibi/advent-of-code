pub fn solve() -> u32 {
	let mut password_count = 0;
	for password in 158126..=624574 {
		if is_valid_password(password) {
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

fn is_valid_password(password: i32) -> bool {
	let digits = to_digits(password);
	let mut i = 0;
	let mut j = i + 1;
	let (mut is_decreasing, mut has_equal_adjacent_values, mut matching_digits_not_part_of_larger_group) =
		(true, false, true);
	loop {
		if j == digits.len() {
			break;
		}
		if digits[i] > digits[j] {
			is_decreasing = false;
			break;
		}
		if digits[i] == digits[j] && !has_equal_adjacent_values {
			if i > 0 && digits[i - 1] == digits[i] {
				matching_digits_not_part_of_larger_group = false;
				i += 1;
				j += 1;
				continue;
			}
			if j + 1 < digits.len() && digits[j] == digits[j + 1] {
				matching_digits_not_part_of_larger_group = false;
				i += 1;
				j += 1;
				continue;
			}
			has_equal_adjacent_values = true;
			matching_digits_not_part_of_larger_group = true;
		}
		i += 1;
		j += 1;
	}
	is_decreasing && has_equal_adjacent_values && matching_digits_not_part_of_larger_group
}
