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
		let (key, value) = (field[0], field[1]);
		match key {
			"byr" => {
				flag = flag ^ (1 << 7);
				if let Some(birth_year) = value.parse::<u16>().ok() {
					if birth_year >= 1920 && birth_year <= 2002 {
						//
					} else {
						return false;
					}
				} else {
					return false;
				}
			}
			"iyr" => {
				flag = flag ^ (1 << 6);
				if let Some(issue_year) = value.parse::<u16>().ok() {
					if issue_year >= 2010 && issue_year <= 2020 {
						//
					} else {
						return false;
					}
				} else {
					return false;
				}
			}
			"eyr" => {
				flag = flag ^ (1 << 5);
				if let Some(expiry) = value.parse::<u16>().ok() {
					if expiry >= 2020 && expiry <= 2030 {
						//
					} else {
						return false;
					}
				} else {
					return false;
				}
			}
			"hgt" => {
				flag = flag ^ (1 << 4);
				let (magnitude, unit) = value.split_at(value.len() - 2);
				match unit {
					"cm" => {
						if let Some(magnitude) = magnitude.parse::<u8>().ok() {
							if magnitude >= 150 && magnitude <= 193 {
								//
							} else {
								return false;
							}
						} else {
							return false;
						}
					}
					"in" => {
						if let Some(magnitude) = magnitude.parse::<u8>().ok() {
							if magnitude >= 59 && magnitude <= 76 {
								//
							} else {
								return false;
							}
						} else {
							return false;
						}
					}
					_catch_all => {
						return false;
					}
				}
			}
			"hcl" => {
				flag = flag ^ (1 << 3);
				if value.starts_with("#")
					&& value.len() == 7
					&& value.chars().skip(1).map(|ch| ch.is_alphanumeric() as u8).sum::<u8>() == 6
				{
					//
				} else {
					return false;
				}
			}
			"ecl" => {
				flag = flag ^ (1 << 2);
				if vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
					//
				} else {
					return false;
				}
			}
			"pid" => {
				flag = flag ^ (1 << 1);
				if value.len() != 9 {
					return false;
				}
				if let Some(_) = value.parse::<u32>().ok() {
					//
				} else {
					return false;
				}
			}
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
