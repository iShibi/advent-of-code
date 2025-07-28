use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let mut memory_banks =
		input_text.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect::<Vec<_>>();
	let mut tracker = HashMap::<String, u32>::new();
	let mut redistribution_cycles = 0;
	loop {
		let mut buffer = memory_banks.iter().copied().enumerate().collect::<Vec<_>>();
		buffer.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));
		let (mut idx, mut blocks_count) = buffer.last().copied().unwrap();
		memory_banks[idx] = 0;
		while blocks_count > 0 {
			idx = (idx + 1) % memory_banks.len();
			memory_banks[idx] += 1;
			blocks_count -= 1;
		}
		redistribution_cycles += 1;
		let state = memory_banks.iter().map(|digit| format!("{digit}")).collect::<String>();
		if let Some(count) = tracker.get(&state) {
			return redistribution_cycles - count;
		} else {
			tracker.insert(state, redistribution_cycles);
		}
	}
}
