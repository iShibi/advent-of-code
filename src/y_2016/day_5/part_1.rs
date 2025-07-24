use md5;

pub fn solve() -> String {
	let input = "reyedfim";
	let mut idx = 0;
	let mut password = String::new();
	loop {
		let key = format!("{}{}", input, idx);
		let hash = format!("{:x}", md5::compute(key));
		if hash.starts_with("00000") {
			let ch = hash.chars().collect::<Vec<_>>();
			password.push(*ch.get(5).unwrap());
		}
		if password.len() == 8 {
			break;
		}
		idx += 1;
	}
	password
}
