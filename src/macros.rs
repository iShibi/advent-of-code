#[macro_export]
macro_rules! input {
	() => {{
		let this_file_name = file!();
		let this_dir_name = std::path::Path::new(this_file_name).parent().unwrap();
		let input_file_name = format!("{}/input.txt", this_dir_name.to_str().unwrap());
		let input_text = std::fs::read_to_string(input_file_name).unwrap();
		input_text
	}};
}
