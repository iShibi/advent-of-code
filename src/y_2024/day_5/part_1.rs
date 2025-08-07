use std::collections::HashMap;

pub fn solve(input_text: &String) -> usize {
	let input = input_text
		.split("\n\n")
		.filter_map(|block| {
			if block.is_empty() {
				return None;
			}
			Some(block.split_terminator('\n').collect::<Vec<_>>())
		})
		.collect::<Vec<_>>();
	let mut rules = HashMap::<usize, Vec<usize>>::new();
	for line in &input[0] {
		let rule = line.split_terminator('|').filter_map(|num| num.parse::<usize>().ok()).collect::<Vec<_>>();
		let (left_num, right_num) = (rule[0], rule[1]);
		if let Some(comes_after) = rules.get_mut(&right_num) {
			comes_after.push(left_num);
		} else {
			rules.insert(right_num, vec![left_num]);
		}
	}
	let mut sum_of_middle_page_num = 0;
	for line in &input[1] {
		let update_order = line.split_terminator(',').filter_map(|num| num.parse::<usize>().ok()).collect::<Vec<_>>();
		let mut is_correctly_ordered = true;
		'check_order: for left_idx in 0..update_order.len() - 1 {
			let left_num = update_order[left_idx];
			if let Some(comes_after) = rules.get(&left_num) {
				for right_idx in left_idx + 1..update_order.len() {
					let right_num = update_order[right_idx];
					if comes_after.contains(&right_num) {
						is_correctly_ordered = false;
						break 'check_order;
					}
				}
			}
		}
		if is_correctly_ordered {
			sum_of_middle_page_num += update_order[update_order.len() / 2];
		}
	}
	sum_of_middle_page_num
}
