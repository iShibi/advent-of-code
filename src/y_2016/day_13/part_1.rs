use std::collections::{HashMap, HashSet, VecDeque};

type Point = (u32, u32);

fn is_wall((x, y): Point, fav_number: u32) -> bool {
	let value = ((x * x) + (3 * x) + (2 * x * y) + y + (y * y)) + fav_number;
	let binary_rep = format!("{:b}", value);
	let parity = binary_rep.split("").filter(|str| *str == "1").count();
	if parity % 2 == 0 { false } else { true }
}

fn get_neighbours((x, y): Point, fav_number: u32) -> Vec<Point> {
	let mut neighbours = vec![];
	if y > 0 {
		let point = (x, y - 1);
		if !is_wall(point, fav_number) {
			neighbours.push(point);
		}
	}
	if x > 0 {
		let point = (x - 1, y);
		if !is_wall(point, fav_number) {
			neighbours.push(point);
		}
	}
	if !is_wall((x, y + 1), fav_number) {
		neighbours.push((x, y + 1));
	}
	if !is_wall((x + 1, y), fav_number) {
		neighbours.push((x + 1, y));
	}
	neighbours
}

pub fn solve() -> usize {
	let fav_number = 1364;
	let start = (1, 1);
	let goal = (31, 39);
	let mut map = HashMap::<Point, Point>::new();
	let mut visited = HashSet::<Point>::new();
	let mut to_be_explored = VecDeque::<Point>::new();
	visited.insert(start);
	to_be_explored.push_front(start);
	while !to_be_explored.is_empty() {
		let current_point = to_be_explored.pop_back().unwrap();
		if current_point == goal {
			break;
		}
		let neighbours = get_neighbours(current_point, fav_number);
		for neighbour in neighbours {
			if !visited.contains(&neighbour) {
				visited.insert(neighbour);
				to_be_explored.push_front(neighbour);
				map.insert(neighbour, current_point);
			}
		}
	}
	let mut path = vec![];
	let mut destination = goal;
	path.push(destination);
	loop {
		let src = map.get(&destination).copied().unwrap();
		path.push(src);
		destination = src;
		if src == start {
			break;
		}
	}
	path.len() - 1
}
