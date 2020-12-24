use std::time::Instant;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let adapters: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).sorted().collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(adapters.clone()));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(adapters.clone()));
	println!("Part 2 time: {:?}", timer.elapsed().as_secs_f64());
}

fn part1(input: Vec<u32>) -> u32 {
	let mut count_1 = 0;
	let mut count_3 = 1;

	let mut input = input;
	input.insert(0, 0);

	input.iter()
		.tuple_windows()
		.for_each(|(p, n)| match n - p {
			1 => count_1 += 1,
			3 => count_3 += 1,
			v => unreachable!(v),
		});

	count_3 * count_1
}

fn part2(input: Vec<u32>) -> usize {
	dp(&input, 0, &mut HashMap::new())
}

fn dp(input: &Vec<u32>, start: usize, stored: &mut HashMap<usize, usize>) -> usize {
	if start == input.len() - 1 {
		return 1;
	}
	if let Some((_, v)) = stored.iter().find(|(i, _)| start == **i) {
		return *v;
	}
	let mut ans = 0;
	for i in (start+1)..input.len() {
		if input[i] - input[start] <= 3 {
			ans += dp(input, i, stored);
		}
	}
	stored.insert(start, ans);
	ans
}
