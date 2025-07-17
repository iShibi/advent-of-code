pub fn solve(input_text: &String) -> i32 {
	let lines = input_text.split_whitespace().collect::<Vec<_>>();
	let mut total_fuel_requirement = 0;
	for line in lines {
		let mass = line.parse::<i32>().unwrap();
		let fuel_requirement = (mass / 3) - 2;
		total_fuel_requirement += fuel_requirement;
	}
	total_fuel_requirement
}
