pub mod y_2019;

fn main() {
	// TODO: this is just a placeholder, create a better system to run solutions
	let args = std::env::args().collect::<Vec<String>>();
	match (args.get(1).map(|s| s.as_str()), args.get(2).map(|s| s.as_str())) {
		(Some("2019"), Some("1")) => println!(
			"Part 1: {}\nPart 2: {}",
			y_2019::day_1::part_1::solve(),
			y_2019::day_1::part_2::solve()
		),
		(Some("2019"), Some("2")) => println!(
			"Part 1: {}\nPart 2: {}",
			y_2019::day_2::part_1::solve(),
			y_2019::day_2::part_2::solve()
		),
		(Some("2019"), Some("3")) => println!(
			"Part 1: {}\nPart 2: {}",
			y_2019::day_3::part_1::solve(),
			y_2019::day_3::part_2::solve()
		),
		(Some("2019"), Some("4")) => println!(
			"Part 1: {}\nPart 2: {}",
			y_2019::day_4::part_1::solve(),
			y_2019::day_4::part_2::solve()
		),
		(Some("2019"), Some("5")) => {
			y_2019::day_5::part_1::solve();
			y_2019::day_5::part_2::solve();
		}
		_ => println!("Usage: cargo run -- <year> <day>"),
	}
}
