use std::fs;

pub fn solve() -> i32 {
	let input_text = fs::read_to_string("src/y_2019/day_1/input.txt").unwrap();
	let lines = input_text.split_whitespace().collect::<Vec<_>>();
	let mut total_fuel_requirement = 0;
	for line in lines {
		let mass = line.parse::<i32>().unwrap();
		let fuel_requirement = (mass / 3) - 2;
		total_fuel_requirement += fuel_requirement;
	}
	total_fuel_requirement
}
