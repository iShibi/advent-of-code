pub mod part_1;
pub mod part_2;

use crate::input;

pub fn solve(part: Option<u8>) {
	let input_text = input!();
	if let Some(part) = part {
		match part {
			1 => {
				part_1::solve(&input_text);
			}
			2 => {
				part_2::solve(&input_text);
			}
			_ => println!("This problem has only two parts."),
		}
	} else {
		part_1::solve(&input_text);
		part_2::solve(&input_text);
	}
}
