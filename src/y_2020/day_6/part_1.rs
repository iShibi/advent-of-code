use std::collections::HashSet;

pub fn solve(input_text: &String) -> usize {
	let groups = input_text.split("\n\n").collect::<Vec<_>>();
	let mut sum = 0;
	for group in groups {
		let set = group
			.split_terminator('\n')
			.flat_map(|person| person.split("").filter(|str| !str.is_empty()))
			.collect::<HashSet<_>>();
		sum += set.iter().count();
	}
	sum
}
