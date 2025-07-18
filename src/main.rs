pub mod macros;
pub mod solution_registry;
pub mod y_2015;
pub mod y_2019;

use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	let solutions = solution_registry::build_registry();
	match (args.get(1), args.get(2)) {
		(Some(year), Some(day)) => {
			if let Some(solution) = solutions.get(&(year, day)) {
				match args.get(3).map(|arg| arg.parse::<u8>().ok()) {
					Some(part) => solution(part),
					None => solution(None),
				}
			} else {
				eprintln!("No solution found for year {year} => day {day}");
			}
		}
		_ => {
			eprintln!("Usage: cargo run -- <year> <day> [part]");
		}
	}
}
