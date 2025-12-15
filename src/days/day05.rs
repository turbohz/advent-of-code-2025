// https://adventofcode.com/2025/day/5

use super::*;

use std::{fmt::Debug, ops::RangeInclusive};

peg::parser! {
	grammar parser() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		pub rule number() -> usize = ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		pub rule range() -> RangeInclusive<usize>
			= r1:number() "-" r2:number() { RangeInclusive::new(r1,r2) }
	}
}

fn parse(input:&str) -> (Vec<RangeInclusive<usize>>,impl Iterator<Item=usize>) {

	let mut lines = input.lines();

	let ranges = lines
		.take_while_ref(|line| !line.is_empty())
		.map(|line| parser::range(line).unwrap())
		.collect_vec();

	let ids = lines.skip(1).map(|line| parser::number(line).unwrap());

	(ranges,ids)
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 5;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let (ranges,mut ids) = parse(input);

		ids.filter(|id| {
			ranges.iter().any(|r| r.contains(id))
		}).count()
	}
}

#[derive(Clone,Copy)]
struct FreshRange {
	fst: usize,
	lst: usize,
}

impl Debug for FreshRange {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let ran:RangeInclusive<usize> = (*self).into();
		ran.fmt(f)
	}
}

impl From<FreshRange> for RangeInclusive<usize> {
	fn from(ran: FreshRange) -> Self {
		ran.fst..=ran.lst
	}
}

impl From<RangeInclusive<usize>> for FreshRange {
	fn from(ran: RangeInclusive<usize>) -> Self {
		Self {
			fst: *ran.start(),
			lst: *ran.end()
		}
	}
}

impl FreshRange {

	fn len(&self) -> usize {
		1 + self.lst - self.fst
	}

	fn unchecked_try_merge(&self,rhs:&FreshRange) -> Option<Self> {
		use std::cmp::max;

		assert!(self.fst <= rhs.fst);

		if (1 + self.lst) >= rhs.fst {
			// Merge
			let lst = max(self.lst,rhs.lst);
			Some(FreshRange { lst, ..*self })
		} else {
			None
		}
	}
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 5;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let (ranges,_) = parse(input);

		let mut ranges = ranges
			.into_iter()
			.map(FreshRange::from)
			.sorted_unstable_by(|a,b| a.fst.cmp(&b.fst));

		#[derive(Debug,Default)]
		struct State {
			acc: usize,
			ran: Option<FreshRange>
		};


		impl State {
			fn into_count(self) -> usize {
				self.acc + self.ran.as_ref().map_or(0, FreshRange::len)
			}
		}

		ranges.fold(State::default(), |mut st,right| {
			if let Some(left) = st.ran {
				if let Some(merged) = left.unchecked_try_merge(&right) {
					st.ran = Some(merged)
				} else {
					// count left and accumulate
					st.acc += left.len();
					st.ran = Some(right);
				}
			} else {
				// First range, initialize st.ran
				st.ran = Some(right)
			}

			st

		}).into_count()
	}
}

#[cfg(test)]
mod test {

	use super::*;

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		3-5
		10-14
		16-20
		12-18

		1
		5
		8
		11
		17
		32
		"#
	};

	#[test]
	fn test_parse() {

		let (ranges,ids) = parse(EXAMPLE_INPUT);

		let expected_ranges = vec![
			3..=5,
			10..=14,
			16..=20,
			12..=18,
		];

		assert_eq!(ranges,expected_ranges);

		let ids:Vec<usize> = ids.collect();
		let expected_ids = vec![1, 5, 8, 11, 17, 32];

		assert_eq!(ids,expected_ids);
	}

	#[test]
	fn test_example() {

		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "3";
		assert_eq!(actual,expected);

		let actual = Part2::solve(EXAMPLE_INPUT).to_string();
		let expected = "14";
		assert_eq!(actual,expected);
	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
