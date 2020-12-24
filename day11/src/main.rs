use crate::Node::{Empty, Occupied, Floor};
use std::time::Instant;
use ndarray::{Array2, Shape, ArrayView2};

#[derive(Debug, Copy, Clone)]
enum Node {
	Empty,
	Occupied,
	Floor,
}

type Grid = Array2<Node>;

trait Parseable {
	fn parse(input: String) -> Self;
}

impl Parseable for Grid {
	fn parse(input: String) -> Self {
		input.lines()
			.map(|l| l.chars()
				.map(|c| match c {
					'L' => Empty,
					'#' => Occupied,
					'.' => Floor,
					_ => unreachable!(),
				}).collect()
			).flatten().collect()
		ArrayView2::from_shape(Sh)
	}
}

fn main() {
	let input = std::fs::read_to_string("input.txt").unwrap();
	let grid = Grid::parse(input);

	let timer = Instant::now();
	println!("Part 1 output: {}", part1(grid.clone()));
	println!("Part 1 time: {:?}", timer.elapsed());
}

fn part1(grid: Grid) -> usize {
	grid.into_iter().count()
}

fn simulate(grid: Grid) -> Grid {
}


