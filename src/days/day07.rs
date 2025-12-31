// https://adventofcode.com/2025/day/7

use std::{fmt::Debug, iter::{Scan, from_fn}, ops::Range, slice::IterMut, vec};

use derive_more::{Deref, DerefMut, From, Index, IndexMut, derive};
use itertools::repeat_n;
use num::traits::float;

use super::{*, Grid as GenericGrid};

type Grid = GenericGrid<Item>;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Item {
	// Initial
	Empty,
	Source,
	Splitter,
	Beam,
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

#[derive(Deref,DerefMut,From)]
struct Manifold(Grid);

impl From<&str> for Manifold {
	fn from(value: &str) -> Self {
		Self(parse(value))
	}
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

/**
The state of a Beam, keeping track of timelines it's been in.
If the beam is split, only one will keep carrying the split count with it,
while the other will forget, to avoid double counting.
*/
#[derive(Clone, Copy)]
struct Beam { splits: usize, timelines: usize }

impl Display for Beam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"[{:02}|{:02}]",self.splits,self.timelines)
	}
}

impl Beam {

	// When beams merges, both beams splits and timeline counts need to be added.
	pub fn merge_with(self, other: &Self) -> Self {
		Beam {
			splits: self.splits + other.splits,
			timelines: self.timelines + other.timelines
		}
	}

	// The left beam is used as "Split carrier", keeping track of splits experienced.
	// Timelines on the other hand, "double".
	pub fn split(self) -> (Self,Self) {
		let left  = Beam { splits: self.splits + 1, ..self };
		let right = Beam { splits: 0, ..self };
		(left,right)
	}
}

impl Default for Beam {
	fn default() -> Self {
		Self { splits: 0, timelines: 1 }
	}
}


#[derive(Index,IndexMut)]
struct ScanLine<T> {
	y: usize,
	#[index]
	#[index_mut]
	items: Vec<Option<T>>
}

impl<T:Display> Display for ScanLine<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		write!(f,"{:02}:",self.y)?;

		for i in self.items.iter() {
			match i {
				Some(beam) => { write!(f,"{}",beam)?; },
				None       => { f.write_str("[__|__]")?; },
			};
		}
		Ok(())
	}
}

impl<T:Copy,S:HasSize> From<&S> for ScanLine<T> {
	fn from(value: &S) -> Self {
		Self::new(value.size().width)
	}
}

impl<T:Copy> ScanLine<T> {
	pub fn new(width:usize) -> Self {
		Self { y: 0, items: vec![None;width] }
	}
}

type BeamFront = ScanLine<Beam>;

struct ManifoldScanner<'a> {
	manifold: &'a mut Manifold,
	beam_front: BeamFront,
}

#[derive(Debug,Copy,Clone)]
enum BeamUpdate {
	Onward(usize),
	Split  { prev: usize, next: (usize,usize) },
}

impl BeamUpdate {
	pub fn prev(&self) -> usize {
		match self {
			Self::Onward(x) => *x,
			Self::Split { prev, .. } => *prev,
		}
	}
}

impl<'a> From<&'a mut Manifold> for ManifoldScanner<'a> {
	fn from(manifold: &'a mut Manifold) -> Self {
		let beam_front = BeamFront::new(manifold.stride());

		Self { manifold, beam_front }
	}
}

impl<'a> ManifoldScanner<'a> {

	pub fn scan(&mut self) -> &BeamFront {
		self.scan_downto(self.manifold.size.height-1)
	}

	pub fn scan_downto(&mut self,downto:usize) -> &BeamFront {

		// println!("{}",self.beam_front);

		for y in 1..=downto {

			let updates = self.manifold.update(&self.beam_front);

			let mut sorted_split_upds = updates.iter()
				.sorted_by(|&a,&b| a.prev().cmp(&b.prev()) );

			for &upd in sorted_split_upds {

				let Some(beam) = self.beam_front[upd.prev()] else {
					assert!(y == 1, "Only in row 1 a Beam can materialize out of nothing!");
					self.beam_front[upd.prev()] = Some(Beam::default());
					break;
				};

				let mut update_beam = |at:usize,beam:Beam| {
					if let Some(target) = self.beam_front[at] {
						// Merge beams
						self.beam_front[at] = Some(target.merge_with(&beam));
					} else {
						// "New" beam
						self.beam_front[at] = Some(beam);
					}
				};

				match upd {

					BeamUpdate::Split { next:(x1,x2), prev } => {
						let (left,right) = beam.split();
						update_beam(x1, left);
						update_beam(x2, right);
						self.beam_front[prev] = None;
					},
					BeamUpdate::Onward(_) => {
						// Nothing to do in this case, beam stays the same
					}
				}
			}

			self.beam_front.y = y;

			// println!("{}",self.beam_front);
		}

		&self.beam_front
	}
}

impl Manifold {

	fn tick<'tick>(&mut self, beam_front:&'tick BeamFront) -> Vec<BeamUpdate> {

		let BeamFront { y: row, items: beams } = beam_front;
		let mut updates:Vec<BeamUpdate> = vec![];

		if *row == 0 {

			debug_assert_eq!(beams.iter().flatten().count(),0);

			let (col,_) = self.find_position(Item::Source)
				.expect("there should be a start point in the top row");

			updates.push(BeamUpdate::Onward(col));

		} else {

			for col in beams.iter().positions(|b| b.is_some()) {

				let prev:Location = (col,*row).into();
				let next:Location = prev.down_unchecked();

				match self[next] {

					Item::Empty => {
						// No update needed, beam stays in the same column
					},
					Item::Splitter => {
						let next = (next.left_unchecked().x, next.right_unchecked().x);
						// Beam splits left and right
						updates.push(BeamUpdate::Split { prev: prev.x, next } )
					},
					_ => {}
				}
			}

		}

		updates
	}

	/// Advances beams, returning the resulting BeamUpdates.
	/// Internally calls [Manifold::tick], and updates the grid.
	pub fn update<'tick>(&mut self, beam_front:&'tick BeamFront) -> Vec<BeamUpdate> {

		use BeamUpdate as BU;

		let y = beam_front.y;

		let updates = self.tick(beam_front);

		let mut place_beam_at = |x:usize| {
			let loc:Location = (x,y).into();
			self[loc] = Item::Beam
		};

		for &update in updates.iter() {

			match update {
				BU::Onward(x) => {
					place_beam_at(x);
				},
				BU::Split { next:(left,right), ..} => {
					place_beam_at(left);
					place_beam_at(right);
				},
			}
		}

		updates
	}

}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 7;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let ref mut manifold:Manifold = input.into();
		let mut scanner:ManifoldScanner = manifold.into();

		let res = scanner.scan();

		res.items.iter()
			.flat_map(|maybe_b| maybe_b.map(|b| b.splits))
			.sum::<usize>()
	}
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 7;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let ref mut manifold:Manifold = input.into();
		let mut scanner:ManifoldScanner = manifold.into();

		let final_beam_front = scanner.scan();

		final_beam_front.items.iter()
			.flat_map(|maybe_b| maybe_b.map(|b| b.timelines))
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

		let actual = Part2::solve(EXAMPLE_INPUT).to_string();
		let expected = "40";

		assert_eq!(actual, expected);

	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
