// https://adventofcode.com/2025/day/5

use super::*;

use std::ops::RangeInclusive;

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
	}

	// SOLUTIONS

	submit! { Part1 }
	// submit! { Part2 }
}
