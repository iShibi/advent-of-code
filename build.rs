use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
	let year_folder = "src/y_2019";
	let registry_file_path = Path::new("src/solution_registry.rs");
	let mut file = File::create(&registry_file_path).unwrap();

	writeln!(
		file,
		r#"use std::collections::HashMap;

pub type SolutionFn = fn(part: Option<u8>);

pub fn build_registry() -> HashMap<(&'static str, &'static str), SolutionFn> {{
	let mut solution_registry = HashMap::new();"#
	)
	.unwrap();

	for entry in fs::read_dir(year_folder).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();

		if path.is_dir() {
			if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
				if name.starts_with("day_") {
					let day = &name[4..];
					writeln!(
						file,
						"	solution_registry.insert((\"2019\", \"{}\"), crate::y_2019::{}::solve as SolutionFn);",
						day, name
					)
					.unwrap()
				}
			}
		}
	}
	writeln!(file, "	solution_registry\n}}").unwrap();
}
