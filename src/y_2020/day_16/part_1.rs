pub fn solve(input_text: &String) -> u32 {
	let notes = input_text.split("\n\n").collect::<Vec<_>>();
	let rules = notes[0]
		.split_whitespace()
		.filter_map(|str| (str.contains('-')).then(|| Rule::new(str)))
		.collect::<Vec<Rule>>();
	let values_from_nearby_tickets = notes[2]
		.split(|ch: char| ch.is_whitespace() || ch == ',')
		.filter_map(|str| str.parse::<u32>().ok())
		.collect::<Vec<_>>();
	let mut ticket_scanning_error_rate = 0;
	for value in values_from_nearby_tickets {
		if !is_valid(value, &rules) {
			ticket_scanning_error_rate += value;
		}
	}
	ticket_scanning_error_rate
}

fn is_valid(value: u32, rules: &Vec<Rule>) -> bool {
	for rule in rules {
		if rule.is_valid(value) {
			return true;
		}
	}
	false
}

#[derive(Debug, Clone, Copy)]
struct Rule {
	start: u32,
	end: u32,
}

impl Rule {
	fn new(range: &str) -> Self {
		let range_values = range.split_terminator('-').filter_map(|str| str.parse::<u32>().ok()).collect::<Vec<_>>();
		Self {
			start: range_values[0],
			end: range_values[1],
		}
	}

	fn is_valid(&self, value: u32) -> bool {
		if value >= self.start && value <= self.end {
			true
		} else {
			false
		}
	}
}
