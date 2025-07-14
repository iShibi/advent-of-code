use std::{collections::HashSet, fs, str::FromStr};

pub fn solve() -> u32 {
	let input_text = fs::read_to_string("src/y_2019/day_3/input.txt").unwrap();
	let wire_paths = input_text.split_whitespace().into_iter().collect::<Vec<_>>();
	let first_wire_path_points = path_to_points(wire_paths[0]);
	let second_wire_path_points = path_to_points(wire_paths[1]);
	let common_points = first_wire_path_points
		.iter()
		.cloned()
		.collect::<HashSet<Point>>()
		.intersection(&second_wire_path_points.iter().cloned().collect::<HashSet<Point>>())
		.cloned()
		.collect::<Vec<Point>>();
	let mut distances = vec![];
	for common_point in common_points {
		let mut distance = 0u32;
		let mut prev_point = (0, 0).into();
		for point in &first_wire_path_points {
			distance += point.distance_from(&prev_point);
			prev_point = point.clone();
			if *point == common_point {
				break;
			}
		}
		prev_point = (0, 0).into();
		for point in &second_wire_path_points {
			distance += point.distance_from(&prev_point);
			prev_point = point.clone();
			if *point == common_point {
				break;
			}
		}
		distances.push(distance);
	}
	distances.iter().min().unwrap().to_owned()
}

enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl FromStr for Direction {
	type Err = String;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"U" => Ok(Direction::Up),
			"R" => Ok(Direction::Right),
			"D" => Ok(Direction::Down),
			"L" => Ok(Direction::Left),
			_ => Err("Not a valid direction".to_string()),
		}
	}
}

// Returns all the points that are traced after taking a step from a given starting point
fn step_to_points(starting_point: Point, step: &str) -> Vec<Point> {
	let Point { x, y } = starting_point;
	let mut chars = step.chars();
	let direction = chars.next().unwrap().to_string().parse::<Direction>().unwrap();
	let magnitude = chars.collect::<String>().parse::<i32>().unwrap();
	let mut points: Vec<Point> = vec![];
	match direction {
		Direction::Up => {
			for i in 1..=magnitude {
				let point = (x, y + i).into();
				points.push(point);
			}
			points
		}
		Direction::Right => {
			for i in 1..=magnitude {
				let point = (x + i, y).into();
				points.push(point);
			}
			points
		}
		Direction::Down => {
			for i in 1..=magnitude {
				let point = (x, y - i).into();
				points.push(point);
			}
			points
		}
		Direction::Left => {
			for i in 1..=magnitude {
				let point = (x - i, y).into();
				points.push(point);
			}
			points
		}
	}
}

// Returns all the points traced by a wire's path
fn path_to_points(path: &str) -> Vec<Point> {
	let steps = path.split(",");
	let mut wire_path_points = vec![];
	let mut starting_point = (0, 0).into();
	for step in steps {
		let points = step_to_points(starting_point, step);
		wire_path_points.append(&mut points.clone());
		starting_point = points.last().unwrap().clone();
	}
	wire_path_points
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}

impl Point {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}

	pub fn distance_from_origin(&self) -> u32 {
		(self.x.abs() as u32) + (self.y.abs() as u32)
	}

	pub fn distance_from(&self, other: &Point) -> u32 {
		if self.x == other.x {
			(self.y - other.y).abs() as u32
		} else if self.y == other.y {
			(self.x - other.x).abs() as u32
		} else {
			panic!("[Out of scope]: Points are on a digonal line.");
		}
	}
}

impl From<(i32, i32)> for Point {
	fn from((x, y): (i32, i32)) -> Self {
		Self { x, y }
	}
}
