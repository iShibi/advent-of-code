pub fn solve(input_text: &String) -> u32 {
	let passports = input_text
		.split("\n\n")
		.filter(|line| !line.is_empty())
		.map(|line| line.split_whitespace().map(|field| field.split(":").collect::<Vec<_>>()).collect::<Vec<_>>())
		.collect::<Vec<_>>();
	let mut valid_passports = 0;
	for passport in passports {
		if is_passport_valid(passport) {
			valid_passports += 1;
		}
	}
	valid_passports
}

fn is_passport_valid(passport: Vec<Vec<&str>>) -> bool {
	let mut flag: u8 = 0b0000_0000;
	for field in passport {
		let key = field[0];
		match key {
			"byr" => flag = flag ^ (1 << 7),
			"iyr" => flag = flag ^ (1 << 6),
			"eyr" => flag = flag ^ (1 << 5),
			"hgt" => flag = flag ^ (1 << 4),
			"hcl" => flag = flag ^ (1 << 3),
			"ecl" => flag = flag ^ (1 << 2),
			"pid" => flag = flag ^ (1 << 1),
			"cid" => flag = flag ^ (1 << 0),
			_catch_all => panic!("'{_catch_all}' is not a valid passport field."),
		}
	}
	if flag == 0b1111_1111 || flag == 0b1111_1110 {
		true
	} else {
		false
	}
}
