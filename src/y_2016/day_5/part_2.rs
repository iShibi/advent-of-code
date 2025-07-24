use md5;
use std::collections::HashMap;

pub fn solve() -> String {
	let input = "reyedfim";
	let mut idx = 0;
	let mut password_map = HashMap::<u8, char>::new();
	loop {
		let key = format!("{}{}", input, idx);
		let hash = format!("{:x}", md5::compute(key));
		if hash.starts_with("00000") {
			let chars = hash.chars().collect::<Vec<_>>();
			let may_be_pos = chars[5].to_string().parse::<u8>().ok();
			if let Some(pos) = may_be_pos {
				if pos <= 7 && !password_map.contains_key(&pos) {
					let char = chars[6];
					password_map.insert(pos, char);
				}
			}
		}
		if password_map.len() == 8 {
			break;
		}
		idx += 1;
	}
	let mut password = vec!["".to_string(); 8];
	for (pos, char) in password_map {
		password[pos as usize] = char.to_string();
	}
	password.join("")
}
