pub fn solve(input_text: &String) -> u32 {
	let mut count = 0;
	let ips = input_text.split_whitespace().collect::<Vec<_>>();
	for ip in ips {
		let sequences = ip.split(|ch| ch == '[' || ch == ']').filter(|str| !str.is_empty()).collect::<Vec<_>>();
		let mut support_tls = false;
		'inner: for idx in 0..sequences.len() {
			if idx % 2 == 0 {
				if sequence_is_abba(sequences[idx]) {
					support_tls = true;
				}
			} else {
				if sequence_is_abba(sequences[idx]) {
					support_tls = false;
					break 'inner;
				}
			}
		}
		if support_tls {
			count += 1;
		}
	}
	count
}

fn sequence_is_abba(sequence: &str) -> bool {
	for i in 0..(sequence.len() - 3) {
		let pair_one = sequence.get(i..=i + 1).unwrap();
		let pair_two = sequence.get((i + 2)..=(i + 3)).unwrap();
		if pair_one.to_string() == pair_two.chars().rev().collect::<String>() && pair_one != pair_two {
			return true;
		}
	}
	false
}
