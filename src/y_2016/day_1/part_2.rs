use std::collections::HashMap;

pub fn solve(input_text: &String) -> i32 {
	let instructions = input_text
		.split(|ch: char| ch.is_whitespace() || ch == ',')
		.filter(|str| !str.is_empty())
		.collect::<Vec<_>>();
	let mut santa = Santa::default();
	let mut path: HashMap<(i32, i32), bool> = HashMap::new();
	for instruction in &instructions {
		let (direction, distance) = instruction.split_at(1);
		santa = santa.walk(direction.into(), distance.parse::<i32>().unwrap(), &mut path);
		if santa.has_reached_destination {
			break;
		}
	}
	santa.x.abs() + santa.y.abs()
}

#[derive(Debug)]
struct Santa {
	x: i32,
	y: i32,
	orientation: Orientation,
	has_reached_destination: bool,
}

impl Default for Santa {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			orientation: Orientation::North,
			has_reached_destination: false,
		}
	}
}

impl Santa {
	fn walk(&self, direction: Direction, distance: i32, path: &mut HashMap<(i32, i32), bool>) -> Self {
		let (current_x, current_y) = (self.x, self.y);
		let new_orientation = self.orientation.turn(direction);
		match new_orientation {
			Orientation::North => {
				for i in 1..=distance {
					let point = (current_x, current_y + i);
					if path.contains_key(&point) {
						return Self {
							x: point.0,
							y: point.1,
							orientation: new_orientation,
							has_reached_destination: true,
						};
					}
					path.insert(point, true);
				}
				return Self {
					x: current_x,
					y: current_y + distance,
					orientation: new_orientation,
					has_reached_destination: false,
				};
			}
			Orientation::South => {
				for i in 1..=distance {
					let point = (current_x, current_y - i);
					if path.contains_key(&point) {
						return Self {
							x: point.0,
							y: point.1,
							orientation: new_orientation,
							has_reached_destination: true,
						};
					}
					path.insert(point, true);
				}
				return Self {
					x: current_x,
					y: current_y - distance,
					orientation: new_orientation,
					has_reached_destination: false,
				};
			}
			Orientation::East => {
				for i in 1..=distance {
					let point = (current_x + i, current_y);
					if path.contains_key(&point) {
						return Self {
							x: point.0,
							y: point.1,
							orientation: new_orientation,
							has_reached_destination: true,
						};
					}
					path.insert(point, true);
				}
				return Self {
					x: current_x + distance,
					y: current_y,
					orientation: new_orientation,
					has_reached_destination: false,
				};
			}
			Orientation::West => {
				for i in 1..=distance {
					let point = (current_x - i, current_y);
					if path.contains_key(&point) {
						return Self {
							x: point.0,
							y: point.1,
							orientation: new_orientation,
							has_reached_destination: true,
						};
					}
					path.insert(point, true);
				}
				return Self {
					x: current_x - distance,
					y: current_y,
					orientation: new_orientation,
					has_reached_destination: false,
				};
			}
		}
	}
}

enum Direction {
	Right,
	Left,
}

impl From<&str> for Direction {
	fn from(value: &str) -> Self {
		match value {
			"L" => Self::Left,
			"R" => Self::Right,
			_catch_all => panic!("Only 'L' and 'R' are the supported directions. Found '{_catch_all}'"),
		}
	}
}

#[derive(Debug)]
enum Orientation {
	North,
	South,
	West,
	East,
}

impl Orientation {
	fn turn(&self, direction: Direction) -> Self {
		match self {
			Self::North => match direction {
				Direction::Left => Self::West,
				Direction::Right => Self::East,
			},
			Self::West => match direction {
				Direction::Left => Self::South,
				Direction::Right => Self::North,
			},
			Self::South => match direction {
				Direction::Left => Self::East,
				Direction::Right => Self::West,
			},
			Self::East => match direction {
				Direction::Left => Self::North,
				Direction::Right => Self::South,
			},
		}
	}
}
