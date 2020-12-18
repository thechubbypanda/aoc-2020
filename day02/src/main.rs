fn main() {
	let mut data: Vec<&str> = vec!();

	let input = std::fs::read_to_string("input.txt").unwrap();
	input.lines().for_each(|l| data.push(l));

	println!("Part 1 output: {}", part1(data));
}

fn part1(data: Vec<&str>) -> u32 {
	let mut count = 0;
	for l in data {
		let pattern_data: Vec<&str> = l.split(": ").collect();
		let pass = pattern_data[1];
		let range_c: Vec<&str> = pattern_data[0].split(" ").collect();
		let range: Vec<&str> = range_c[0].split("-").collect();
		let test = range_c[1];
		let min = range[0].parse().unwrap();
		let max = range[1].parse().unwrap();

		let mut char_count = 0;
		for c in pass.chars() {
			if c == test.parse().unwrap() {
				char_count += 1;
			}
		}

		if char_count <= max && char_count >= min {
			count += 1;
		}
	}
	return count;
}
