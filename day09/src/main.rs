use std::time::Instant;

const PREAMBLE: usize = 25;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let sequence: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(sequence.clone()));
	println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(input: Vec<u64>) -> u64 {
	'main: for (i, num) in input.iter().enumerate().skip(PREAMBLE) {
		for (j, f) in input.iter()
			.skip(i - PREAMBLE)
			.take(PREAMBLE)
			.filter_map(|v| if num >= v { Some(num - v)} else { None })
			.enumerate() {
			for (k, p) in input.iter()
				.skip(i - PREAMBLE)
				.take(PREAMBLE)
				.enumerate() {
				if j == k {
					continue;
				}
				if f==*p {
					continue 'main;
				}
			}
		}
		return *num;
	}
	panic!("None")
}
