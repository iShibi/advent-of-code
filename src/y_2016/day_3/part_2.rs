pub fn solve(input_text: &String) -> u32 {
	let triangles = input_text
		.split("\n")
		.filter(|str| !str.is_empty())
		.map(|str| str.split_whitespace().filter_map(|str| str.parse::<u32>().ok()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut valid_triangles = 0;
	for col in 0..=2 {
		for i in (0..triangles.len()).step_by(3) {
			let a = triangles[i][col];
			let b = triangles[i + 1][col];
			let c = triangles[i + 2][col];
			if (a + b <= c) || (b + c <= a) || (a + c <= b) {
				continue;
			}
			valid_triangles += 1;
		}
	}
	valid_triangles
}
