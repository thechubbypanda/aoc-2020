use std::time::Instant;
use itertools::Itertools;

const PREAMBLE: usize = 25;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let sequence: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(sequence.clone()));
	println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(input: Vec<u64>) -> u64 {
	input.as_slice()
		.windows(PREAMBLE + 1)
		.find(|win| win[0..PREAMBLE].iter()
			.tuple_combinations()
			.all(|(a,b)| a + b != win[PREAMBLE])
		).unwrap()[0]
}
