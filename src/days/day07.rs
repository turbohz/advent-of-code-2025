// https://adventofcode.com/2025/day/7

use std::iter::from_fn;

use derive_more::{Deref, DerefMut};
use itertools::repeat_n;

use super::{*, Grid as GenericGrid};

type Grid = GenericGrid<Item>;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Item {
	// Initial
	Empty,
	Source,
	Splitter,
	// Dynamic
	Beam,
	BeamFront,
}

impl Display for Item {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let b:&[u8;1] = &[(*self).into()];
		f.write_str(unsafe { str::from_utf8_unchecked(b) })
	}
}

impl TryFrom<u8> for Item {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			b'.' => Ok(Self::Empty),
			b'S' => Ok(Self::Source),
			b'^' => Ok(Self::Splitter),
			b'|' => Ok(Self::Beam),
			b'*' => Ok(Self::BeamFront),
			_ => Err(())
		}
	}
}

impl From<Item> for u8 {
	fn from(value: Item) -> Self {
		match value {
			Item::Empty     => b'.',
			Item::Source    => b'S',
			Item::Splitter  => b'^',
			Item::Beam      => b'|',
			Item::BeamFront => b'*',
		}
	}
}

fn parse(input:&str) -> Grid {

	let parse_line = |l:&str| -> Vec<Item> {
		l.as_bytes().iter().map(|&b| b.try_into().unwrap()).collect_vec()
	};

	let mut items = input.lines().map(parse_line).peekable();
	let width = items.peek().unwrap().len();
	let items = items.flatten().collect_vec();
	Grid::new(width,items)
}

#[derive(Deref,DerefMut)]
struct Manifold(Grid);

impl From<&str> for Manifold {
	fn from(value: &str) -> Self {
		Self(parse(value))
	}
}

struct Update {
	at: Location,
	it: Item,
}

impl Display for Manifold {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let stride = self.stride();
		for chunk in self.iter().chunks(stride).into_iter() {
			let row = chunk.map(Item::to_string).collect_vec().join("");
			writeln!(f,"{}",row)?;
		}
		std::fmt::Result::Ok(())
	}
}

impl Manifold {

	pub fn new(mut grid:Grid) -> Self {

		let source_loc:Location = grid.find_position(Item::Source).unwrap().into();

		// Add Beamfront right below

		let beamfront_loc:Location = source_loc.down_unchecked();
		grid[beamfront_loc] = Item::BeamFront;

		Self(grid)
	}

	pub fn tick(&mut self) -> Option<usize> {

		let mut updates:Vec<Update> = vec![];

		#[allow(clippy::filter_map_bool_then)]
		let mut beamfronts = self.enumerate().filter_map(|(xy,&i)| {
			matches!(i,Item::BeamFront).then(|| (xy,i))
		}).collect_vec();

		let mut splits = 0;

		if beamfronts.is_empty() {
			return None
		}

		// Move beamfronts

		for (xy,breamfront) in beamfronts  {

			let next:Location = Location::from(xy).down_unchecked();

			if next.y < self.size.height {

				match self[next] {

					Item::Empty => {
						// Advance
						updates.push(Update { at: next , it: Item::BeamFront });
					},
					Item::Splitter => {
						splits += 1;
						// Split
						updates.push(Update { at: next.left_unchecked() , it: Item::BeamFront });
						updates.push(Update { at: next.right_unchecked(), it: Item::BeamFront });
					},
					_ => {}
				}
			}

			// not a beamfront anymore

			updates.push(Update { at: xy.into(), it: Item::Beam });
		}

		// Update grid

		for Update { at, it } in updates {
			self[at] = it;
		}

		Some(splits)
	}
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 7;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let mut manifold = Manifold::new(parse(input));

		from_fn(|| manifold.tick())
			.sum::<usize>()
	}
}

#[cfg(test)]
mod test {

	use super::*;

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		.......S.......
		...............
		.......^.......
		...............
		......^.^......
		...............
		.....^.^.^.....
		...............
		....^.^...^....
		...............
		...^.^...^.^...
		...............
		..^...^.....^..
		...............
		.^.^.^.^.^...^.
		...............
		"#
	};

	#[test]
	fn test() {

		let grid = parse(EXAMPLE_INPUT);

		assert_eq!(grid[(0,0)], Item::Empty);
		assert_eq!(grid[(7,0)], Item::Source);
		assert_eq!(grid[(7,2)], Item::Splitter);
	}

	#[test]
	fn test_example() {

		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "21";

		assert_eq!(actual, expected);
	}

	// SOLUTIONS

	submit! { Part1 }
}
