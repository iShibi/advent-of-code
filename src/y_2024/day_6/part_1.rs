use std::collections::{HashMap, HashSet};

pub fn solve(input_text: &String) -> usize {
	let area = input_text
		.split_terminator('\n')
		.map(|row| row.split("").filter(|location| !location.is_empty()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut guard = Guard::default();
	let mut tracker = HashMap::<(usize, usize), bool>::new();
	for row in 0..area.len() {
		for col in 0..area[0].len() {
			let item = area[row][col];
			match item {
				"#" => {
					tracker.insert((col, row), true);
				}
				"^" => {
					guard.x = col as isize;
					guard.y = row as isize;
					tracker.insert((col, row), false);
				}
				"." => {
					tracker.insert((col, row), false);
				}
				_catch_all => panic!("'{_catch_all}' is not valid item"),
			}
		}
	}
	let mut visited_positions = HashSet::<(usize, usize)>::new();
	visited_positions.insert((guard.x as usize, guard.y as usize));
	loop {
		let (next_x, next_y) = guard.add_direction_vector(guard.direction.into());
		if next_x >= 0 && next_y >= 0 && next_x < area[0].len() as isize && next_y < area.len() as isize {
			if let Some(is_obstacle) = tracker.get(&(next_x as usize, next_y as usize)) {
				if *is_obstacle {
					match guard.direction {
						Direction::North => guard.direction = Direction::East,
						Direction::East => guard.direction = Direction::South,
						Direction::South => guard.direction = Direction::West,
						Direction::West => guard.direction = Direction::North,
					};
				} else {
					guard.x = next_x;
					guard.y = next_y;
					visited_positions.insert((guard.x as usize, guard.y as usize));
				}
			}
		} else {
			break;
		}
	}
	visited_positions.len()
}

#[derive(Debug, Clone, Copy)]
struct Guard {
	x: isize,
	y: isize,
	direction: Direction,
}

impl Default for Guard {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			direction: Direction::North,
		}
	}
}

impl Guard {
	fn add_direction_vector(&self, (delta_x, delta_y): (isize, isize)) -> (isize, isize) {
		(self.x + delta_x, self.y + delta_y)
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl Into<(isize, isize)> for Direction {
	fn into(self) -> (isize, isize) {
		match self {
			Self::North => (0, -1),
			Self::East => (1, 0),
			Self::South => (0, 1),
			Self::West => (-1, 0),
		}
	}
}
