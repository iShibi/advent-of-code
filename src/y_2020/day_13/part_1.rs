use std::usize;

pub fn solve(input_text: &String) -> usize {
	let notes = input_text.split_terminator('\n').collect::<Vec<_>>();
	let earliest_departure_time = notes[0].parse::<usize>().unwrap();
	let bus_ids = notes[1].split(',').filter_map(|id| id.parse::<usize>().ok()).collect::<Vec<_>>();
	let mut least_wait_time = usize::MAX;
	let mut earliest_bus_id = usize::MAX;
	for id in bus_ids {
		let departed_minutes_ago = earliest_departure_time % id;
		if departed_minutes_ago == 0 {
			least_wait_time = 0;
			earliest_bus_id = id;
			break;
		} else {
			let wait_time = id - departed_minutes_ago;
			if wait_time < least_wait_time {
				least_wait_time = wait_time;
				earliest_bus_id = id;
			}
		}
	}
	earliest_bus_id * least_wait_time
}
