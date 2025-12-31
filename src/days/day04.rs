//! https://adventofcode.com/2025/day/4

use std::ops::{Index, IndexMut, RangeInclusive};
use derive_more::IsVariant;
use num::integer::div_rem;

use super::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq,IsVariant)]
enum Cell {
	Empty,
	Roll
}

trait Swap {
	fn swap(self) -> Self;
}

impl<T> Swap for (T,T) {
	fn swap(self) -> Self {
		(self.1,self.0)
	}
}

struct Grid {
	size: (usize,usize),
	data: Vec<Cell>
}

impl Index<usize> for Grid {
	type Output = Cell;

	fn index(&self, index: usize) -> &Self::Output {
		&self.data[index]
	}
}

impl IndexMut<usize> for Grid {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.data[index]
	}
}

impl Grid {

	fn new(input: &str) -> Self {
		let mut lines = parse(input,parser::grid_line);
		let mut data = lines.next().unwrap();
		let width = data.len();
		let mut height = 1;
		for mut l in lines {
			height += 1;
			data.append(&mut l);
		}
		Self { size:(width,height), data }
	}

	#[inline]
	fn last(&self)->usize {
		self.data.len()-1
	}

	#[allow(unused)]
	fn coordinate(&self,i:usize)->(usize,usize) {
		div_rem(i, self.size.0).swap()
	}

	fn range(&self) -> RangeInclusive<usize> {
		0..=self.last()
	}

	/// Returns the upper and lower bound for the row
	/// that contains index i
	fn row(&self, i:usize) -> RangeInclusive<usize> {
		let (width,_) = self.size;
		let offset_x = i % width;
		let left = i-offset_x;
		left..=(left+width-1)
	}

	fn roll_locations(&self) -> impl Iterator<Item=usize> {
		self.range().filter(|&i| self[i].is_roll())
	}

	fn row_neighbours(&self, i:usize) -> impl Iterator<Item=usize> {
		let row_bounds = self.row(i);
		let (min,max) = (*row_bounds.start(), *row_bounds.end());
		[
			i.saturating_sub(1).clamp(min,max),
			i,
			i.saturating_add(1).clamp(min,max)
		].into_iter()
	}

	fn neighbours(&self,i:usize) -> impl Iterator<Item=usize> {
		let (width,_) = self.size;


		let up   = i.checked_sub(width).unwrap_or(i);
		let down = {
			let v = i + width;
			if v <= self.last() { v } else { i }
		};

		itertools::chain!(
				self.row_neighbours(up),
				self.row_neighbours(i),
				self.row_neighbours(down)
			)
			// rows can be repeated in first and last rows
			.unique()
			// exclude 'i'
			.filter(move |&v| v != i)
	}

	fn has_reachable_roll_at(&self,i:usize) -> bool {
		self.neighbours(i).filter(|&i| self[i].is_roll()).count() < 4
	}
}

peg::parser! {

	grammar parser() for str {
		rule roll()  -> Cell = "@" { Cell::Roll }
		rule empty() -> Cell = "." { Cell::Empty }
		pub rule grid_line() -> Vec<Cell> =
			cs:((roll()/empty())+) { cs }
	}

}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 4;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let grid = Grid::new(input);

		grid.roll_locations().filter(|&i| {
			grid.has_reachable_roll_at(i)
		}).count()
	}
}
struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 4;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let mut grid = Grid::new(input);

		let mut total_removed = 0;

		// TODO: keep state for known roll locations

		loop {

			let to_be_removed = grid.roll_locations()
				.filter(|&i| grid.has_reachable_roll_at(i))
				.collect_vec();

			if to_be_removed.is_empty() {
				break total_removed;
			} else {
				to_be_removed.iter().for_each(|&i| { grid[i] = Cell::Empty });
				total_removed += to_be_removed.len();
			}
		}
	}
}
#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		..@@.@@@@.
		@@@.@.@.@@
		@@@@@.@.@@
		@.@@@@..@.
		@@.@@@@.@@
		.@@@@@@@.@
		.@.@.@.@@@
		@.@@@.@@@@
		.@@@@@@@@.
		@.@.@@@.@.
		"#
	};

	#[test]
	fn test_example() {

		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "13";

		assert_eq!(actual,expected);

		let actual = Part2::solve(EXAMPLE_INPUT).to_string();
		let expected = "43";

		assert_eq!(actual,expected)
	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
