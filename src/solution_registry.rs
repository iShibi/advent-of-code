use std::collections::HashMap;

pub type SolutionFn = fn(part: Option<u8>);

pub fn build_registry() -> HashMap<(&'static str, &'static str), SolutionFn> {
	let mut solution_registry = HashMap::new();
	solution_registry.insert(("2015", "1"), crate::y_2015::day_1::solve as SolutionFn);
	solution_registry.insert(("2015", "12"), crate::y_2015::day_12::solve as SolutionFn);
	solution_registry.insert(("2015", "2"), crate::y_2015::day_2::solve as SolutionFn);
	solution_registry.insert(("2015", "4"), crate::y_2015::day_4::solve as SolutionFn);
	solution_registry.insert(("2015", "5"), crate::y_2015::day_5::solve as SolutionFn);
	solution_registry.insert(("2015", "7"), crate::y_2015::day_7::solve as SolutionFn);
	solution_registry.insert(("2019", "1"), crate::y_2019::day_1::solve as SolutionFn);
	solution_registry.insert(("2019", "10"), crate::y_2019::day_10::solve as SolutionFn);
	solution_registry.insert(("2019", "2"), crate::y_2019::day_2::solve as SolutionFn);
	solution_registry.insert(("2019", "3"), crate::y_2019::day_3::solve as SolutionFn);
	solution_registry.insert(("2019", "4"), crate::y_2019::day_4::solve as SolutionFn);
	solution_registry.insert(("2019", "5"), crate::y_2019::day_5::solve as SolutionFn);
	solution_registry.insert(("2019", "6"), crate::y_2019::day_6::solve as SolutionFn);
	solution_registry.insert(("2019", "7"), crate::y_2019::day_7::solve as SolutionFn);
	solution_registry.insert(("2019", "8"), crate::y_2019::day_8::solve as SolutionFn);
	solution_registry.insert(("2019", "9"), crate::y_2019::day_9::solve as SolutionFn);
	solution_registry
}
