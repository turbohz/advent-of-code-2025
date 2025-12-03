// https://adventofcode.com/2025/day/1

use super::*;
use num::Zero;

enum Dir { L, R }

struct Stride {
	dir: Dir,
	dis: usize
}

impl From<Stride> for isize {
	fn from(Stride { dir, dis } : Stride) -> Self {
		let res = dis as isize;
		if matches!(dir,Dir::L) { -res } else { res }
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
		let t = isize::from(s);
		let v = self.0 as isize;
		self.0 = (v+t).rem_euclid(100) as usize;
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

		let zeroes = strides
			.map(|s| { dial.turn(s); dial.value() })
			.filter(Zero::is_zero)
			.count();

		zeroes
	}
}

#[cfg(test)]
mod test {

	use super::*;

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
		let actual:Vec<isize> = turns.map(Into::into).collect();

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

		assert_eq!(actual,expected)
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		Part1::try_submit()?;
		Ok(())
	}
}
