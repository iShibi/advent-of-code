pub fn solve(input_text: &String) -> usize {
	let memory = input_text.clone();
	let mut result = 0;
	let mut instruction = String::new();
	let mut is_currently_parsing_mul = false;
	for ch in memory.chars() {
		match ch {
			'm' => {
				if is_currently_parsing_mul {
					instruction = String::new();
				} else {
					is_currently_parsing_mul = true;
				}
				instruction.push(ch);
			}
			')' => {
				if is_currently_parsing_mul {
					instruction.push(ch);
					is_currently_parsing_mul = false;
					if instruction.starts_with("mul(") {
						let operands = instruction
							.split(|ch| ch == '(' || ch == ',' || ch == ')')
							.filter_map(|token| token.parse::<usize>().ok())
							.collect::<Vec<_>>();
						if operands.len() == 2 {
							result += operands[0] * operands[1];
						}
					}
					instruction = String::new();
				}
			}
			_ => {
				if is_currently_parsing_mul && (['m', 'u', 'l', '(', ','].contains(&ch) || ch.is_digit(10)) {
					instruction.push(ch);
				} else {
					is_currently_parsing_mul = false;
					instruction = String::new();
				}
			}
		}
	}
	result
}
