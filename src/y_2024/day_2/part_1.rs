pub fn solve(input_text: &String) -> usize {
	let reports = input_text
		.split_terminator('\n')
		.map(|line| line.split_whitespace().filter_map(|str| str.parse::<usize>().ok()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut safe_reports_count = 0;
	for report in reports {
		let mut trend = get_trend(report[0], report[1]);
		match trend {
			Trend::Flat => {
				continue;
			}
			_ => {
				let mut is_safe = true;
				'check: for idx in 0..report.len() - 1 {
					let left_level = report[idx];
					let right_level = report[idx + 1];
					let abs_diff = left_level.abs_diff(right_level);
					if abs_diff < 1 || abs_diff > 3 {
						is_safe = false;
						break 'check;
					}
					let new_trend = get_trend(left_level, right_level);
					if new_trend != trend {
						is_safe = false;
						break 'check;
					} else {
						trend = new_trend;
					}
				}
				if is_safe {
					safe_reports_count += 1;
				}
			}
		}
	}
	safe_reports_count
}

#[derive(Debug, PartialEq, Eq)]
enum Trend {
	Inc,
	Dec,
	Flat,
}

fn get_trend(left_level: usize, right_level: usize) -> Trend {
	if left_level > right_level {
		Trend::Dec
	} else if right_level > left_level {
		Trend::Inc
	} else {
		Trend::Flat
	}
}
