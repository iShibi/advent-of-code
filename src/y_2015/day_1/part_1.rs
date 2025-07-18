pub fn solve(input: &String) -> isize {
	let instructions = input.split("").filter(|c| *c == ")" || *c == "(").collect::<Vec<_>>();
	let mut current_floor = 0;
	for instruction in instructions {
		match instruction {
			"(" => current_floor += 1,
			")" => current_floor -= 1,
			_catch_all => panic!("Only '(' and ')' instructions are supported. Found '{_catch_all}'"),
		}
	}
	current_floor
}
