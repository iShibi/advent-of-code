pub fn solve(input_text: &String) -> u32 {
	let triangles = input_text
		.split("\n")
		.filter(|str| !str.is_empty())
		.map(|str| str.split_whitespace().filter_map(|str| str.parse::<u32>().ok()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut valid_triangles = 0;
	for triangle in triangles {
		let (a, b, c) = (triangle[0], triangle[1], triangle[2]);
		if (a + b <= c) || (b + c <= a) || (a + c <= b) {
			continue;
		}
		valid_triangles += 1;
	}
	valid_triangles
}
