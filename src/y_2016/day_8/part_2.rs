use std::collections::HashMap;

pub fn solve(input_text: &String) {
	let (screen_width, screen_height) = (50, 6);
	let mut screen = HashMap::new();
	let mut screen_buffer = HashMap::new();
	let instructions = input_text.split_terminator('\n').collect::<Vec<_>>();
	for instruction in instructions {
		let tokens = instruction.split_whitespace().collect::<Vec<_>>();
		let opcode = tokens[0];
		match opcode {
			"rect" => {
				let dimensions = tokens[1].split("x").filter_map(|str| str.parse::<u32>().ok()).collect::<Vec<_>>();
				let (rec_width, rec_height) = (dimensions[0], dimensions[1]);
				for x in 0..rec_width {
					for y in 0..rec_height {
						let pixel_pos = (x, y);
						screen_buffer.insert(pixel_pos, true);
					}
				}
			}
			"rotate" => {
				let axis = tokens[1];
				match axis {
					"column" => {
						let col = tokens[2].split("=").filter_map(|str| str.parse::<u32>().ok()).collect::<Vec<_>>()[0];
						let rotate_by = tokens[4].parse::<u32>().unwrap();
						let mut buffer = vec![];
						for (x, y) in screen.keys().copied() {
							if x == col {
								screen_buffer.remove(&(x, y));
								let new_pixel_position = (x, (y + rotate_by) % screen_height);
								buffer.push(new_pixel_position);
							}
						}
						for pixel in buffer {
							screen_buffer.insert(pixel, true);
						}
					}
					"row" => {
						let row = tokens[2].split("=").filter_map(|str| str.parse::<u32>().ok()).collect::<Vec<_>>()[0];
						let rotate_by = tokens[4].parse::<u32>().unwrap();
						let mut buffer = vec![];
						for (x, y) in screen.keys().copied() {
							if y == row {
								screen_buffer.remove(&(x, y));
								let new_pixel_position = ((x + rotate_by) % screen_width, y);
								buffer.push(new_pixel_position);
							}
						}
						for pixel in buffer {
							screen_buffer.insert(pixel, true);
						}
					}
					_catch_all => panic!("Only 'column' and 'row' axises are supported. Found '{_catch_all}'"),
				}
			}
			_catch_all => panic!("Only 'rect' and 'rotate' opcodes are supported. Found '{_catch_all}'"),
		}
		screen = screen_buffer.clone();
	}
	let row = vec!["  "; screen_width as usize];
	let mut display = vec![row; screen_height as usize];
	for (x, y) in screen.keys() {
		display[*y as usize][*x as usize] = "##";
	}
	for row in display {
		println!("{}", row.join(""));
	}
}
