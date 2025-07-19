use md5;

pub fn solve() -> usize {
	let input_text = "yzbqklnj".to_string();
	let mut num = 0;
	let mut hash;
	loop {
		num += 1;
		let key = format!("{}{}", input_text, num);
		hash = format!("{:x}", md5::compute(key));
		if hash.starts_with("00000") {
			break;
		}
	}
	num
}
