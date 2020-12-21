use std::time::Instant;
use std::collections::HashSet;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let input = input.split("\n\n").collect();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(&input));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(&input));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: &Vec<&str>) -> usize {
	input.iter().map(|group| {
		let mut set: HashSet<char> = HashSet::new();
		for l in group.lines() {
			for c in l.chars() {
				set.insert(c);
			}
		}
		set.len()
	}).fold(0, |a, v| a + v)
}

fn part2(input: &Vec<&str>) -> usize {
	input.iter().map(|group| {
		let mut set: HashSet<char> = HashSet::new();
		for l in group.lines() {
			for c in l.chars() {
				set.insert(c);
			}
		}
		set.iter()
			.map(|c| {
				group.lines()
					.map(|l| l.contains(*c))
					.fold(true, |a, v| a && v)
			})
            .filter(|v| *v)
            .count()
	}).fold(0, |a, v| a + v)
}
