use std::collections::{HashMap, HashSet};

pub fn solve(input_text: &String) -> usize {
	let orbits = input_text.split_whitespace().collect::<Vec<_>>();
	let mut orbit_tracker = HashMap::<String, Vec<String>>::new();
	let mut total_orbit_counts = 0;
	loop {
		for orbit in &orbits {
			let objects = orbit.split(")").collect::<Vec<_>>();
			let (parent_body, sattelite) = (objects[0].to_string(), objects[1].to_string());
			if let Some(indirect_parents) = orbit_tracker.get(&parent_body) {
				let mut parents = vec![parent_body.clone()];
				parents.extend(indirect_parents.iter().cloned());
				orbit_tracker.insert(sattelite, parents);
			} else {
				orbit_tracker.insert(sattelite, vec![parent_body]);
			}
		}
		let current_orbits_count = orbit_tracker
			.values()
			.map(|v| v.iter().count())
			.collect::<Vec<_>>()
			.iter()
			.sum::<usize>();
		if total_orbit_counts == current_orbits_count {
			break;
		} else {
			total_orbit_counts = current_orbits_count;
		}
	}
	let you_parents: HashSet<_> = orbit_tracker.get("YOU").unwrap().iter().collect();
	let santa_parents: HashSet<_> = orbit_tracker.get("SAN").unwrap().iter().collect();
	let unique_parents: Vec<_> = you_parents
		.symmetric_difference(&santa_parents)
		.cloned()
		.collect();
	unique_parents.len()
}
