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
	let mut idx = 0;
	while idx < first_list.len() {
		similarity_score += first_list[idx].abs_diff(second_list[idx]);
		idx += 1;
	}
	similarity_score
}
