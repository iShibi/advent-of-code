use std::collections::HashMap;

pub fn solve(input_text: &String) -> String {
	let instructions = input_text.split_whitespace().collect::<Vec<_>>();
	let keypad_map: HashMap<(i32, i32), KeypadButton> = HashMap::from([
		(KeypadButton::One.into(), KeypadButton::One),
		(KeypadButton::Two.into(), KeypadButton::Two),
		(KeypadButton::Three.into(), KeypadButton::Three),
		(KeypadButton::Four.into(), KeypadButton::Four),
		(KeypadButton::Five.into(), KeypadButton::Five),
		(KeypadButton::Six.into(), KeypadButton::Six),
		(KeypadButton::Seven.into(), KeypadButton::Seven),
		(KeypadButton::Eight.into(), KeypadButton::Eight),
		(KeypadButton::Nine.into(), KeypadButton::Nine),
		(KeypadButton::A.into(), KeypadButton::A),
		(KeypadButton::B.into(), KeypadButton::B),
		(KeypadButton::C.into(), KeypadButton::C),
		(KeypadButton::D.into(), KeypadButton::D),
	]);
	let mut bathroom_code = String::new();
	let mut current_button = KeypadButton::Five;
	for instruction in &instructions {
		for ch in instruction.chars() {
			let (current_x, current_y) = current_button.into();
			match ch {
				'U' => {
					let try_postion = (current_x, current_y - 1);
					if let Some(value) = keypad_map.get(&try_postion) {
						current_button = value.clone();
					}
				}
				'D' => {
					let try_postion = (current_x, current_y + 1);
					if let Some(value) = keypad_map.get(&try_postion) {
						current_button = value.clone();
					}
				}
				'L' => {
					let try_postion = (current_x - 1, current_y);
					if let Some(value) = keypad_map.get(&try_postion) {
						current_button = value.clone();
					}
				}
				'R' => {
					let try_postion = (current_x + 1, current_y);
					if let Some(value) = keypad_map.get(&try_postion) {
						current_button = value.clone();
					}
				}
				_catch_all => panic!("Only the instruction U, D, L, and R are supported. Found {_catch_all}"),
			}
		}
		bathroom_code.push(current_button.into());
	}
	bathroom_code
}

#[derive(Debug, Clone, Copy)]
pub enum KeypadButton {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	A,
	B,
	C,
	D,
}

impl From<KeypadButton> for (i32, i32) {
	fn from(value: KeypadButton) -> Self {
		match value {
			KeypadButton::One => (2, 0),
			KeypadButton::Two => (1, 1),
			KeypadButton::Three => (2, 1),
			KeypadButton::Four => (3, 1),
			KeypadButton::Five => (0, 2),
			KeypadButton::Six => (1, 2),
			KeypadButton::Seven => (2, 2),
			KeypadButton::Eight => (3, 2),
			KeypadButton::Nine => (4, 2),
			KeypadButton::A => (1, 3),
			KeypadButton::B => (2, 3),
			KeypadButton::C => (3, 3),
			KeypadButton::D => (2, 4),
		}
	}
}

impl From<KeypadButton> for char {
	fn from(value: KeypadButton) -> Self {
		match value {
			KeypadButton::One => '1',
			KeypadButton::Two => '2',
			KeypadButton::Three => '3',
			KeypadButton::Four => '4',
			KeypadButton::Five => '5',
			KeypadButton::Six => '6',
			KeypadButton::Seven => '7',
			KeypadButton::Eight => '8',
			KeypadButton::Nine => '9',
			KeypadButton::A => 'A',
			KeypadButton::B => 'B',
			KeypadButton::C => 'C',
			KeypadButton::D => 'D',
		}
	}
}
