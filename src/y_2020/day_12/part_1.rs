pub fn solve(input_text: &String) -> isize {
	let instructions = input_text
		.split_terminator('\n')
		.map(|line| {
			let (action, value) = line.split_at(1);
			(action, value.parse::<isize>().unwrap())
		})
		.collect::<Vec<_>>();
	let mut ship = Ship::default();
	for (action, value) in instructions {
		match action {
			"N" => {
				ship.y -= value;
			}
			"S" => {
				ship.y += value;
			}
			"E" => {
				ship.x += value;
			}
			"W" => {
				ship.x -= value;
			}
			"L" => {
				ship.turn_left(value);
			}
			"R" => {
				ship.turn_right(value);
			}
			"F" => match ship.direction {
				Direction::North => ship.y -= value,
				Direction::South => ship.y += value,
				Direction::East => ship.x += value,
				Direction::West => ship.x -= value,
			},
			_catch_all => panic!("'{_catch_all}' is not a valid action."),
		}
	}
	ship.x + ship.y
}

#[derive(Debug, Clone, Copy)]
struct Ship {
	x: isize,
	y: isize,
	direction: Direction,
}

impl Default for Ship {
	fn default() -> Self {
		Self {
			x: 0,
			y: 0,
			direction: Direction::East,
		}
	}
}

impl Ship {
	fn turn_right(&mut self, degrees: isize) {
		let num_of_turns = degrees / 90;
		let current_direction: isize = self.direction.into();
		let new_direction = (current_direction + num_of_turns) % 4;
		self.direction = new_direction.into();
	}

	fn turn_left(&mut self, degrees: isize) {
		let num_of_turns = degrees / 90;
		let num_of_turns_right = 4 - (num_of_turns % 4);
		let current_direction: isize = self.direction.into();
		let new_direction = (current_direction + num_of_turns_right) % 4;
		self.direction = new_direction.into();
	}
}

#[derive(Debug, Clone, Copy)]
enum Direction {
	North,
	East,
	South,
	West,
}

impl From<Direction> for isize {
	fn from(value: Direction) -> Self {
		match value {
			Direction::North => 0,
			Direction::East => 1,
			Direction::South => 2,
			Direction::West => 3,
		}
	}
}

impl From<isize> for Direction {
	fn from(value: isize) -> Self {
		match value {
			0 => Self::North,
			1 => Self::East,
			2 => Self::South,
			3 => Self::West,
			_ => panic!("Never"),
		}
	}
}
