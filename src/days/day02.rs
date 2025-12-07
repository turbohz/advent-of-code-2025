// https://adventofcode.com/2025/day/2

use super::*;

use std::ops::RangeInclusive;

peg::parser! {
	grammar parser() for str {

		rule digit() -> char = [c if c.is_ascii_digit()]

		rule number() -> usize = ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		rule range() -> RangeInclusive<usize>
			= r1:number() "-" r2:number() { RangeInclusive::new(r1,r2) }

		pub rule ranges() -> Vec<RangeInclusive<usize>>
			= rs:(range() ++ ",") { rs }
	}
}

fn has_eq_chunks_of(str:&str, chunk_size:usize) -> bool {

	let len = str.len();

	// Chunk size has to be multiple of size

	len % chunk_size == 0 && {

		// ... and first chunk is repeated all along

		let mut repeats_upto:usize = 0;

		let chars:Vec<char> = str.chars().collect();

		for i in 0..str.len() {
			if chars[i%chunk_size] == chars[i] {
				repeats_upto = i
			} else {
				break;
			}
		}

		repeats_upto == len-1
	}

}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 2;
	const PART: Part = Part::Part1;


	fn solve(input:&str) -> impl Display {

		use num::Integer;

		let ranges = parse(input,parser::ranges).next().unwrap();

		let mut total:usize = 0;

		for r in ranges {

			for v in r {

				let s = v.to_string();

				let (div,rem) = s.len().div_rem(&2);
				let can_halve = rem == 0;

				if can_halve && has_eq_chunks_of(&s,div) {
					total += v
				}
			}
		}

		total
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const EXAMPLE_INPUT:&str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

	#[test]
	fn test_part1_example() {

		let expected = "1227775554";
		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		assert_eq!(actual,expected);
	}

	#[test]
	fn test_parse() {

		let actual:Vec<RangeInclusive<usize>> = parse(EXAMPLE_INPUT,parser::ranges).take(1).next().unwrap();

		let expected:Vec<RangeInclusive<usize>> = vec![
			11..=22, 95..=115, 998..=1012, 1188511880..=1188511890, 222220..=222224,
			1698522..=1698528, 446443..=446449, 38593856..=38593862, 565653..=565659,
			824824821..=824824827, 2121212118..=2121212124
		];

		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		Part1::try_submit()?;
		Ok(())
	}
}
