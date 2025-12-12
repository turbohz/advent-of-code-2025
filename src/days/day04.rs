//! https://adventofcode.com/2025/day/4

use std::{num::Saturating, ops::{Index, RangeBounds, RangeInclusive}};
use derive_more::IsVariant;
use num::{Bounded, clamp, integer::div_rem};

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

	fn row_neighbours(&self, i:usize)-> impl Iterator<Item=usize> {
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

		let reachable = |i:usize| -> bool {
			let rolls = grid.neighbours(i)
				.filter(|&i| grid[i].is_roll())
				.count();
			rolls < 4
		};

		grid.range().filter(|&i| {
			grid[i].is_roll() && reachable(i)
		}).count()
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

		assert_eq!(actual,expected)
	}

	// SOLUTIONS

	submit! { Part1 }
}
