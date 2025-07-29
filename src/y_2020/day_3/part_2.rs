pub fn solve(input_text: &String) -> usize {
	let area = input_text
		.split_terminator('\n')
		.map(|line| line.chars().filter(|ch| !ch.is_whitespace()).collect::<Vec<char>>())
		.collect::<Vec<_>>();
	let mut tree_count_product = 1;
	let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	for (right, down) in slopes {
		tree_count_product *= get_tree_count_for_slope(right, down, &area);
	}
	tree_count_product
}

fn get_tree_count_for_slope(right: usize, down: usize, area: &Vec<Vec<char>>) -> usize {
	let mut tree_count = 0;
	let (mut x, mut y) = (0, 0);
	let area_width = area[0].len();
	while y != area.len() - 1 {
		x = (x + right) % area_width;
		y += down;
		if area[y][x] == '#' {
			tree_count += 1;
		}
	}
	tree_count
}
