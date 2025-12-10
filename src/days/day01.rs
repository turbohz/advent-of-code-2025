// https://adventofcode.com/2025/day/1

use super::*;
use num::{Zero, zero};

#[derive(Clone, Copy)]
enum Dir { L, R }

#[derive(Clone, Copy)]
struct Stride {
	dir: Dir,
	dis: usize
}

impl Stride {
	fn as_raw(&self) -> isize {
		let res = self.dis as isize;
		if matches!(self.dir,Dir::L) { -res } else { res }
	}

	fn into_raw(self) -> isize {
		self.as_raw()
	}

	fn from_raw(d:isize) -> Stride {
		let dir = if d >= 0 { Dir::R } else { Dir::L };
		Stride { dir, dis: d.unsigned_abs() }
	}
}

struct Dial(usize);

impl Default for Dial {
	fn default() -> Self {
		Self(50)
	}
}

impl Dial {

	fn turn(&mut self, s:Stride) {

		let s = s.as_raw();
		let v = self.0 as isize;


		self.0 = (v+s).rem_euclid(100) as usize;
	}

	fn value(&self) -> usize {
		self.0
	}
}

peg::parser! {
	grammar parser() for str {
		rule dir() -> Dir
			= d:['L'|'R'] {
				match d {
					'L' => Dir::L,
					'R' => Dir::R,
					_ => unreachable!()
				}}

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule dis() -> usize
			= n:$(digit()+) {? n.parse().or(Err("Expected usize value")) }

		pub rule stride() -> Stride
			= dir:dir()dis:dis() { Stride { dir, dis } }
	}
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 1;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let mut dial = Dial::default();
		let strides = parse(input,parser::stride);

		strides
			.map(|s| {
				dial.turn(s);
				dial.value()
			})
			.filter(Zero::is_zero)
			.count()
	}
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 1;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let mut dial = Dial::default();
		let strides = parse(input,parser::stride);

		let mut zero_crossed:usize = 0;

		// Returns a "reduced" stride, and the amount of full revolutions removed
		fn simplify(s:Stride) -> (Stride,usize) {
			(Stride { dis: s.dis % 100,..s },s.dis / 100)
		}

		for s in strides {

			let (s, full_revs) = simplify(s);

			// Account for full rotations crossing by zero
			zero_crossed += full_revs;

			let prev = dial.value();

			dial.turn(s);

			let next = dial.value();

			// Any simplified stride that started from 0 cannot **cross** by zero.
			// Otherwise ..
			if prev > 0 {

				let inc = if next == 0 {
					1
				} else {

					match s.dir {
						Dir::L if next > prev => 1,
						Dir::R if next < prev => 1,
						_ => 0
					}
				};

				zero_crossed += inc;
			}
		}

		zero_crossed
	}
}

#[cfg(test)]
mod test {

	use super::*;
	use rstest::*;

	const EXAMPLE_INPUT:&str = indoc! {
	r#"
		L68
		L30
		R48
		L5
		R60
		L55
		L1
		L99
		R14
		L82
	"# };

	#[test]
	fn test_parse() {

		let turns = parse(EXAMPLE_INPUT,parser::stride);

		let expected = vec![ -68, -30, 48, -5, 60, -55, -1,-99, 14, -82 ];
		let actual:Vec<isize> = turns.map(Stride::into_raw).collect();

		assert_eq!(actual,expected);
	}

	#[test]
	fn test_dial() {

		let mut dial = Dial::default();

		let strides = parse(EXAMPLE_INPUT,parser::stride);

		let expected = vec![ 82, 52, 0, 95, 55, 0, 99, 0, 14, 32 ];

		let actual:Vec<_> = strides.map(|s| {
			dial.turn(s);
			dial.value()
		}).collect();

		assert_eq!(actual,expected);
	}

	#[test]
	fn test_example() {

		let expected = "3";
		let actual = Part1::solve(EXAMPLE_INPUT).to_string();

		assert_eq!(actual,expected);

		let expected = "6";
		let actual = Part2::solve(EXAMPLE_INPUT).to_string();

		assert_eq!(actual,expected);
	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
