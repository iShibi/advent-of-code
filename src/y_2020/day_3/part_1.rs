pub fn solve(input_text: &String) -> u32 {
	let area = input_text
		.split_terminator('\n')
		.map(|line| line.chars().filter(|ch| !ch.is_whitespace()).collect::<Vec<char>>())
		.collect::<Vec<_>>();
	let mut tree_count = 0;
	let (mut x, mut y) = (0, 0);
	let area_width = area[0].len();
	while y != area.len() - 1 {
		x = (x + 3) % area_width;
		y += 1;
		if area[y][x] == '#' {
			tree_count += 1;
		}
	}
	tree_count
}
