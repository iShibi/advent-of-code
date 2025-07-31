use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Vector4D {
	x: i32,
	y: i32,
	z: i32,
	w: i32,
}

impl Vector4D {
	fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
		Self { x, y, z, w }
	}
}

pub fn solve(input_text: &String) -> usize {
	let initial_region = input_text
		.split_whitespace()
		.map(|line| line.split("").filter(|str| !str.is_empty()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut tracker = HashMap::<Vector4D, bool>::new();
	for (y, row) in initial_region.iter().enumerate() {
		for (x, value) in row.iter().enumerate() {
			if *value == "#" {
				tracker.insert(
					Vector4D {
						x: x as i32,
						y: y as i32,
						z: 0,
						w: 0,
					},
					true,
				);
			}
		}
	}
	let cycles = 6;
	for _ in 1..=cycles {
		let mut buffer = HashMap::<Vector4D, bool>::new();
		for (point, _) in &tracker {
			if should_remain_active(&point, &tracker) {
				buffer.insert(point.clone(), true);
			}
			let neighbours: Vec<Vector4D> = get_neighbours(&point);
			for neighbour in neighbours {
				if !tracker.contains_key(&neighbour) {
					if should_be_active(&neighbour, &tracker) {
						buffer.insert(neighbour, true);
					}
				}
			}
		}
		tracker = buffer.clone();
	}
	tracker.keys().count()
}

fn should_remain_active(point: &Vector4D, tracker: &HashMap<Vector4D, bool>) -> bool {
	let mut count = 0;
	let neighbours = get_neighbours(point);
	for neighbour in &neighbours {
		if tracker.contains_key(neighbour) {
			count += 1;
		}
	}
	if count == 2 || count == 3 { true } else { false }
}

fn should_be_active(point: &Vector4D, tracker: &HashMap<Vector4D, bool>) -> bool {
	let mut count = 0;
	let neighbours = get_neighbours(point);
	for neighbour in &neighbours {
		if tracker.contains_key(neighbour) {
			count += 1;
		}
	}
	if count == 3 { true } else { false }
}

fn get_neighbours(point: &Vector4D) -> Vec<Vector4D> {
	let mut neighbours = vec![];
	let Vector4D { x, y, z, w } = point.to_owned();
	for xi in [x - 1, x, x + 1] {
		for yi in [y - 1, y, y + 1] {
			for zi in [z - 1, z, z + 1] {
				for wi in [w - 1, w, w + 1] {
					if xi == x && yi == y && zi == z && wi == w {
						continue;
					}
					neighbours.push(Vector4D::new(xi, yi, zi, wi));
				}
			}
		}
	}
	neighbours
}
