use std::collections::{HashMap, VecDeque};

pub fn solve() -> usize {
	let puzzle_input = "15,5,1,4,7,0";
	let initial_numbers =
		puzzle_input.split_terminator(',').filter_map(|str| str.parse::<usize>().ok()).collect::<Vec<_>>();
	let mut tracker = HashMap::<usize, VecDeque<usize>>::new();
	for (idx, num) in initial_numbers.iter().enumerate() {
		let mut stack = VecDeque::new();
		stack.push_front(idx + 1);
		tracker.insert(*num, stack);
	}
	let mut turn = initial_numbers.len() + 1;
	let mut prev_num = *initial_numbers.last().unwrap();
	loop {
		let prev_turns_stack = tracker.get(&prev_num).unwrap();
		if prev_turns_stack.len() > 1 {
			let recent_turn = prev_turns_stack[0];
			let older_turn = prev_turns_stack[1];
			let age = recent_turn - older_turn;
			if let Some(turn_stack) = tracker.get_mut(&age) {
				turn_stack.push_front(turn);
			} else {
				let mut turn_stack = VecDeque::new();
				turn_stack.push_front(turn);
				tracker.insert(age, turn_stack);
			}
			prev_num = age;
			if turn == 2020 {
				return age;
			}
		} else {
			if let Some(turn_stack) = tracker.get_mut(&0) {
				turn_stack.push_front(turn);
			} else {
				let mut turn_stack = VecDeque::new();
				turn_stack.push_front(turn);
				tracker.insert(0, turn_stack);
			}
			prev_num = 0;
			if turn == 2020 {
				return 0;
			}
		}
		turn += 1;
	}
}
