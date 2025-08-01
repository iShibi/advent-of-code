pub fn solve(input_text: &String) -> u32 {
	let mut joltage_ratings =
		input_text.split_terminator('\n').filter_map(|line| line.parse::<u32>().ok()).collect::<Vec<_>>();
	joltage_ratings.sort();
	let outlet_joltage = 0;
	let mut one_jolt_diffs = 0;
	let mut three_jolt_diffs = 0;
	if joltage_ratings[0] - outlet_joltage == 1 {
		one_jolt_diffs += 1;
	} else if joltage_ratings[0] - outlet_joltage == 3 {
		three_jolt_diffs += 1;
	}
	for idx in 0..(joltage_ratings.len() - 1) {
		let left_jolt = joltage_ratings[idx];
		let right_jolt = joltage_ratings[idx + 1];
		if right_jolt - left_jolt == 1 {
			one_jolt_diffs += 1;
		} else if right_jolt - left_jolt == 3 {
			three_jolt_diffs += 1;
		}
	}
	three_jolt_diffs += 1;
	one_jolt_diffs * three_jolt_diffs
}
