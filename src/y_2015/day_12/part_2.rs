use serde_json::Value;

pub fn solve(input_text: &String) -> i64 {
	let json_doc: Value = serde_json::from_str(&input_text).unwrap();
	get_sum_of_numbers(&json_doc)
}

fn get_sum_of_numbers(json_doc: &Value) -> i64 {
	let mut sum = 0;
	for (_, value) in json_doc.as_object().unwrap() {
		if value == "red" {
			return 0;
		} else if value.is_object() {
			sum += get_sum_of_numbers(value);
		} else if value.is_i64() {
			sum += value.as_i64().unwrap();
		} else if value.is_array() {
			sum += get_sum_of_numbers_from_arr(value);
		}
	}
	sum
}

fn get_sum_of_numbers_from_arr(arr: &Value) -> i64 {
	let mut sum = 0;
	for item in arr.as_array().unwrap() {
		if item.is_i64() {
			sum += item.as_i64().unwrap();
		} else if item.is_object() {
			sum += get_sum_of_numbers(item);
		} else if item.is_array() {
			sum += get_sum_of_numbers_from_arr(item);
		}
	}
	sum
}
