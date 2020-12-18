use std::time::Instant;

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(&input, (3, 1)));
	println!("Part 1 time: {:?}", timer.elapsed());
	let timer = Instant::now();
	println!("Part 2 output: {}", part2(&input));
	println!("Part 2 time: {:?}", timer.elapsed());
}

fn part1(input: &String, (x, y): (usize, usize)) -> usize {
	let mut current = 0;
	return input.lines()
		.step_by(y)
		.enumerate()
		.map(|l| -> Vec<char> {
			l.1.chars().collect()
		})
		.map(|row| -> usize {
			current += x;
			if row[(current - x) % row.len()] == '#' {
				return 1;
			}
			return 0;
		})
		.fold(0, |count, v| count + v);
}

fn part2(input: &String) -> usize {
	let mut totals = vec!();
	totals.push(part1(input, (1, 1)));
	totals.push(part1(input, (3, 1)));
	totals.push(part1(input, (5, 1)));
	totals.push(part1(input, (7, 1)));
	totals.push(part1(input, (1, 2)));
	return totals.iter().fold(1, |a, v| a * v);
}
