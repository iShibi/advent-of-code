use std::collections::HashMap;

pub fn solve(input_text: &String) -> String {
	let list = input_text.split_terminator('\n').collect::<Vec<_>>();
	let mut programs_with_no_parent = HashMap::new();
	let mut child_to_parent_map = HashMap::new();
	for item in list {
		let (parent_data, child_programs) = item
			.split_terminator(|ch: char| ch.is_whitespace() || ch == '-' || ch == '>' || ch == ',')
			.filter(|str| !str.is_empty())
			.enumerate()
			.partition::<Vec<_>, _>(|(idx, _)| *idx < 2)
			.pipe(|(parent, children)| {
				(
					parent.iter().map(|(_, v)| v.to_owned()).collect::<Vec<_>>(),
					children.iter().map(|(_, v)| v.to_owned()).collect::<Vec<_>>(),
				)
			});
		let program = parent_data[0];
		if !child_to_parent_map.contains_key(program) {
			programs_with_no_parent.insert(program, true);
		}
		for child_program in child_programs {
			child_to_parent_map.insert(child_program, program);
			programs_with_no_parent.remove(child_program);
		}
	}
	programs_with_no_parent.keys().nth(0).unwrap().to_string()
}

trait Pipe: Sized {
	fn pipe<F, R>(self, f: F) -> R
	where
		F: FnOnce(Self) -> R,
	{
		f(self)
	}
}
impl<T> Pipe for T {}
