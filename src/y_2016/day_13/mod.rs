pub mod part_1;
// pub mod part_2;

pub fn solve(part: Option<u8>) {
	if let Some(part) = part {
		match part {
			1 => {
				println!("Part 1: {}", part_1::solve());
			}
			2 => {
				// println!("Part 2: {}", part_2::solve());
			}
			_ => println!("This problem has only two parts."),
		}
	} else {
		println!("Part 1: {}", part_1::solve());
		// println!("Part 2: {}", part_2::solve());
	}
}
