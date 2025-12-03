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

		let mut turns = parse(input,parser::stride).map(isize::from);

		let zeroes = std::iter::successors(Some(50), |v| {
			turns.next().map(|t| (v+t).rem_euclid(100))
		}).filter(Zero::is_zero).count();

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
