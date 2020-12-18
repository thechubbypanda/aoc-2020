use std::time::Instant;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let passports: Vec<&str> = input.split("\n\n").collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(passports));
	println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(input: Vec<&str>) -> usize {
	input.iter()
		.map(|passport| FIELDS.iter()
			.map(|field| passport.contains(field))
			.fold(true, |a, v| a && v)
		)
		.filter(|valid| *valid)
		.count()
}
