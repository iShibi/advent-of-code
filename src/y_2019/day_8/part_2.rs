pub fn solve(input_text: &String) {
	let digits = input_text.split("").filter_map(|c| c.parse::<u8>().ok()).collect::<Vec<u8>>();
	let (image_width, image_height) = (25, 6);
	let mut digit_count = 0;
	let mut layers = vec![];
	let mut layer = vec![];
	for digit in digits {
		digit_count += 1;
		layer.push(digit);
		if digit_count == (image_width * image_height) {
			layers.push(layer);
			layer = vec![];
			digit_count = 0
		}
	}
	let mut image = vec![2; image_width * image_height];
	for layer in layers {
		for (idx, pixel) in layer.iter().enumerate() {
			if image[idx] == 2 {
				image[idx] = *pixel;
			}
		}
	}
	println!("Part 2:");
	for chunk in image.chunks(image_width) {
		for pixel in chunk {
			match pixel {
				0 => print!(".."),
				1 => print!("00"),
				_ => {
					// Do Nothing
				}
			}
		}
		print!("\n");
	}
}
