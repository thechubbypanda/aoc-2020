use std::time::Instant;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(adapters.clone()).ok().unwrap());
	println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(mut input: Vec<u32>) -> Result<u32, u32> {
	input.sort();

	let mut count_1 = 0;
	let mut count_3 = 1;
	let mut last = 0;

	for a in input {
		match a - last {
			1 => {
				last = a;
				count_1 += 1;
			}
			3 => {
				last = a;
				count_3 += 1;
			}
			e => return Err(e),
		}
	}

	Ok(count_3 * count_1)
}
