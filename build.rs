use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
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

	let src_folder = "src/";
	for year in fs::read_dir(src_folder).unwrap() {
		let year_folder = year.unwrap().path();
		if year_folder.is_file() {
			continue;
		}
		if let Some(year_folder_name) = year_folder.file_name().and_then(|n| n.to_str()) {
			if year_folder_name.starts_with("y_") {
				let year = &year_folder_name[2..];
				for day in fs::read_dir(year_folder.as_path()).unwrap() {
					let day_folder = day.unwrap().path();
					if day_folder.is_file() {
						continue;
					}
					if let Some(day_folder_name) = day_folder.file_name().and_then(|n| n.to_str()) {
						if day_folder_name.starts_with("day_") {
							let day = &day_folder_name[4..];
							writeln!(
								file,
								"	solution_registry.insert((\"{year}\", \"{day}\"), crate::{year_folder_name}::{day_folder_name}::solve as SolutionFn);"
							)
							.unwrap()
						}
					}
				}
			}
		}
	}
	writeln!(file, "	solution_registry\n}}").unwrap();
}
