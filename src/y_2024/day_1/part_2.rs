use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let lists = input_text
		.split_terminator('\n')
		.map(|line| {
			let mut numbers = line.split_whitespace().filter_map(|str| str.parse::<usize>().ok());
			(numbers.next().unwrap(), numbers.next().unwrap())
		})
		.collect::<Vec<_>>();
	let mut first_list = lists.iter().map(|(first_num, _)| *first_num).collect::<Vec<_>>();
	let mut second_list = lists.iter().map(|(_, second_num)| *second_num).collect::<Vec<_>>();
	first_list.sort();
	second_list.sort();
	let mut similarity_score = 0;
	let mut occurence_tracker = HashMap::<usize, usize>::new();
	for first_num in &first_list {
		let mut occurence = 0;
		if let Some(count) = occurence_tracker.get(first_num) {
			occurence = *count;
		} else {
			for second_num in &second_list {
				if second_num == first_num {
					occurence += 1;
				}
			}
			occurence_tracker.insert(*first_num, occurence);
		}
		similarity_score += first_num * occurence;
	}
	similarity_score
}
