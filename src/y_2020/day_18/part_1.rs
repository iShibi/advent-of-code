use std::collections::VecDeque;

pub fn solve(input_text: &String) -> usize {
	let expressions = input_text.split_terminator('\n').collect::<Vec<_>>();
	let mut sum = 0;
	for expression in expressions {
		let tokens = parse(expression);
		let mut numbers_stack = VecDeque::<usize>::new();
		let mut operators_stack = VecDeque::<Operators>::new();
		for token in tokens {
			match token {
				Tokens::Number(num) => {
					if numbers_stack.len() >= 1 {
						let prev_num = numbers_stack.pop_front().unwrap();
						let operator = operators_stack.pop_front().unwrap();
						match operator {
							Operators::Plus => {
								numbers_stack.push_front(prev_num + num);
							}
							Operators::Minus => {
								numbers_stack.push_front(prev_num - num);
							}
							Operators::Multiply => {
								numbers_stack.push_front(prev_num * num);
							}
							Operators::Divide => {
								numbers_stack.push_front(prev_num / num);
							}
							Operators::OpenParen => {
								numbers_stack.push_front(prev_num);
								numbers_stack.push_front(num);
							}
							Operators::CloseParen => panic!("Never"),
						}
					} else {
						numbers_stack.push_front(num);
					}
				}
				Tokens::Op(Operators::CloseParen) => {
					if numbers_stack.len() >= 2 {
						let right_num = numbers_stack.pop_front().unwrap();
						let left_num = numbers_stack.pop_front().unwrap();
						let operator = operators_stack.pop_front().unwrap();
						match operator {
							Operators::Plus => {
								numbers_stack.push_front(left_num + right_num);
							}
							Operators::Minus => {
								numbers_stack.push_front(left_num - right_num);
							}
							Operators::Multiply => {
								numbers_stack.push_front(left_num * right_num);
							}
							Operators::Divide => {
								numbers_stack.push_front(left_num / right_num);
							}
							Operators::OpenParen => {
								numbers_stack.push_front(left_num);
								numbers_stack.push_front(right_num);
							}
							Operators::CloseParen => panic!("Never"),
						}
					}
				}
				Tokens::Op(operator) => {
					operators_stack.push_front(operator);
				}
			}
		}
		sum += numbers_stack.pop_front().unwrap();
	}
	sum
}

fn parse(expression: &str) -> Vec<Tokens> {
	let chars = expression.chars().collect::<Vec<_>>();
	let mut tokens = vec![];
	let mut idx = 0;
	while idx < chars.len() {
		let char = chars[idx];
		match char {
			'(' => tokens.push(Tokens::Op(Operators::OpenParen)),
			'+' => tokens.push(Tokens::Op(Operators::Plus)),
			'-' => tokens.push(Tokens::Op(Operators::Minus)),
			'*' => tokens.push(Tokens::Op(Operators::Multiply)),
			'/' => tokens.push(Tokens::Op(Operators::Divide)),
			')' => tokens.push(Tokens::Op(Operators::CloseParen)),
			_catch_all => {
				if _catch_all.is_digit(10) {
					let mut num: usize = _catch_all.to_digit(10).unwrap() as usize;
					'create_num: loop {
						idx += 1;
						if let Some(maybe_next_digit) = (idx < chars.len()).then(|| chars[idx].to_digit(10)) {
							if let Some(next_digit) = maybe_next_digit {
								num = (num * 10) + next_digit as usize;
							} else {
								tokens.push(Tokens::Number(num));
								break 'create_num;
							}
						} else {
							tokens.push(Tokens::Number(num));
							break 'create_num;
						}
					}
					continue;
				} else {
					idx += 1;
					continue;
				}
			}
		}
		idx += 1;
	}
	tokens
}

#[derive(Debug, Clone, Copy)]
enum Tokens {
	Number(usize),
	Op(Operators),
}

#[derive(Debug, Clone, Copy)]
enum Operators {
	Plus,
	Minus,
	Multiply,
	Divide,
	OpenParen,
	CloseParen,
}
