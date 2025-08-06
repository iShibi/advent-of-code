use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let list = input_text.split_terminator('\n').map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
	let mut floor = HashMap::<HexTile, bool>::new();
	for line in list {
		let mut tile = HexTile::default();
		let mut idx = 0;
		while idx < line.len() {
			let ch = line[idx];
			match ch {
				'e' => {
					// east
					tile.q += 1;
					idx += 1;
				}
				'w' => {
					// west
					tile.q -= 1;
					idx += 1;
				}
				'n' => {
					if line[idx + 1] == 'e' {
						// ne
						tile.q += 1;
						tile.r -= 1;
					} else {
						// nw
						tile.r -= 1;
					}
					idx += 2;
				}
				's' => {
					if line[idx + 1] == 'e' {
						// se
						tile.r += 1;
					} else {
						// sw
						tile.q -= 1;
						tile.r += 1;
					}
					idx += 2;
				}
				_catch_all => panic!("'{_catch_all}' is not a valid step."),
			}
		}
		if let Some(is_black) = floor.get_mut(&tile) {
			*is_black = !*is_black;
		} else {
			floor.insert(tile, true);
		}
	}
	floor.values().filter(|is_black| **is_black).count()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct HexTile {
	q: isize,
	r: isize,
}

impl Default for HexTile {
	fn default() -> Self {
		Self { q: 0, r: 0 }
	}
}
