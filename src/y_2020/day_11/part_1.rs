use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let layout = input_text
		.split_terminator('\n')
		.map(|line| line.split("").filter(|str| !str.is_empty()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut tracker = HashMap::<Seat, bool>::new();
	let height = layout.len();
	let width = layout[0].len();
	for row in 0..height {
		for col in 0..width {
			match layout[row][col] {
				"L" => {
					tracker.insert((col, row).into(), false);
				}
				"#" => {
					tracker.insert((col, row).into(), true);
				}
				_catch_all => (),
			}
		}
	}
	loop {
		let mut changes = 0;
		let mut buffer = HashMap::<Seat, bool>::new();
		for (seat, is_occupied) in &tracker {
			let neighbours = get_neighbours(seat, (width, height));
			match *is_occupied {
				true => {
					let mut occupied_adjacent_seats = 0;
					for neighbour in &neighbours {
						if let Some(is_occupied) = tracker.get(neighbour) {
							if *is_occupied {
								occupied_adjacent_seats += 1;
							}
						}
					}
					if occupied_adjacent_seats >= 4 {
						buffer.insert(*seat, false);
						changes += 1;
					} else {
						buffer.insert(*seat, true);
					}
				}
				false => {
					let mut has_no_occupied_adjacent_seats = true;
					'check: for neighbour in &neighbours {
						if let Some(is_occupied) = tracker.get(neighbour) {
							if *is_occupied {
								has_no_occupied_adjacent_seats = false;
								break 'check;
							}
						}
					}
					if has_no_occupied_adjacent_seats {
						buffer.insert(*seat, true);
						changes += 1;
					} else {
						buffer.insert(*seat, false);
					}
				}
			}
		}
		tracker = buffer.clone();
		if changes == 0 {
			break;
		}
	}
	tracker.values().filter(|is_occupied| **is_occupied).count()
}

fn get_neighbours(seat: &Seat, (width, height): (usize, usize)) -> Vec<Seat> {
	let mut neighbours: Vec<Seat> = vec![];
	let (current_x, current_y) = (seat.x, seat.y);
	if current_x > 0 {
		neighbours.push(Seat {
			x: current_x - 1,
			y: current_y,
		});
	}
	if current_x + 1 < width {
		neighbours.push(Seat {
			x: current_x + 1,
			y: current_y,
		});
	}
	if current_y > 0 {
		neighbours.push(Seat {
			x: current_x,
			y: current_y - 1,
		});
	}
	if current_y + 1 < height {
		neighbours.push(Seat {
			x: current_x,
			y: current_y + 1,
		});
	}
	if current_x > 0 && current_y > 0 {
		neighbours.push(Seat {
			x: current_x - 1,
			y: current_y - 1,
		});
	}
	if current_x + 1 < width && current_y > 0 {
		neighbours.push(Seat {
			x: current_x + 1,
			y: current_y - 1,
		});
	}
	if current_x > 0 && current_y + 1 < height {
		neighbours.push(Seat {
			x: current_x - 1,
			y: current_y + 1,
		});
	}
	if current_x + 1 < width && current_y + 1 < height {
		neighbours.push(Seat {
			x: current_x + 1,
			y: current_y + 1,
		});
	}
	neighbours
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Seat {
	x: usize,
	y: usize,
}

impl From<(usize, usize)> for Seat {
	fn from((x, y): (usize, usize)) -> Self {
		Seat { x, y }
	}
}
