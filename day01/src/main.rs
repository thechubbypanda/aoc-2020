fn main() {
	let mut data: Vec<u32> = vec!();

	std::fs::read_to_string("input.txt")
		.unwrap()
		.lines()
		.for_each(|l| data.push(l.parse().unwrap()));

	println!("Part 1 output: {}", part1(&data).unwrap());
	println!("Part 2 output: {}", part2(&data).unwrap());
}

fn part1(input: &Vec<u32>) -> Result<u32, &'static str> {
	for i in input.iter() {
		for j in input.iter() {
			let test = i + j;
			if test == 2020 {
				return Ok(i * j);
			}
		}
	}
	return Err("No pair");
}

fn part2(input: &Vec<u32>) -> Result<u32, &'static str> {
	for i in input.iter() {
		for j in input.iter() {
			for k in input.iter() {
				if i + j + k == 2020 {
					return Ok(i * j * k);
				}
			}
		}
	}
	return Err("No pair");
}