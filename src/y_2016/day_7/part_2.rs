use std::collections::HashMap;

pub fn solve(input_text: &String) -> u32 {
	let mut count = 0;
	let ips = input_text.split_whitespace().collect::<Vec<_>>();
	for ip in ips {
		let sequences = ip.split(|ch| ch == '[' || ch == ']').filter(|str| !str.is_empty()).collect::<Vec<_>>();
		let mut aba_tracker = HashMap::new();
		let mut bab_tracker = HashMap::new();
		for idx in 0..sequences.len() {
			let sequence = sequences[idx];
			let valid_triplets = get_valid_triplets(sequence);
			let is_even = idx % 2 == 0;
			if is_even {
				for triplets in valid_triplets {
					aba_tracker.insert(triplets, true);
				}
			} else {
				for triplets in valid_triplets {
					bab_tracker.insert(triplets, true);
				}
			}
		}
		'check: for aba in aba_tracker.keys() {
			let bab = aba_to_bab(aba);
			if bab_tracker.contains_key(bab.as_str()) {
				count += 1;
				break 'check;
			}
		}
	}
	count
}

fn get_valid_triplets(sequence: &str) -> Vec<String> {
	let mut valid_triplets = vec![];
	for i in 0..(sequence.len() - 2) {
		let triplet = sequence.get(i..=i + 2).unwrap().to_string();
		if triplet == triplet.chars().rev().collect::<String>() && triplet.chars().nth(0) != triplet.chars().nth(1) {
			valid_triplets.push(triplet);
		}
	}
	valid_triplets
}

fn aba_to_bab(triplets: &str) -> String {
	let mut chars = triplets.chars();
	let a = chars.nth(0).unwrap();
	let b = chars.nth(0).unwrap();
	format!("{b}{a}{b}")
}
