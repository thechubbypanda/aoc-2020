use std::time::Instant;
use itertools::Itertools;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).sorted().collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(adapters.clone()));
	println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(input: Vec<u32>) -> u32 {
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
			_ => unreachable!(),
		}
	}

	count_3 * count_1
}
