//! https://adventofcode.com/2025/day/3

use super::*;
use derive_more::{Deref, From};

#[derive(Debug)]
#[derive(From,Deref)]
struct BatteryBank(Vec<u8>);

impl BatteryBank {
	// NOTICE: the built-in `iter::max` function returns the maybe last max value,
	// but we want the first (leftmost).
	fn leftmost_max_val_at(&self) -> (usize,u8) {

		assert!(self.len() > 0);

		self.iter().copied().enumerate().fold((0,0), |(first_max_at, max_val),(at,val)| {
			if val > max_val {
				(at,val)
			} else {
				(first_max_at,max_val)
			}
		})
	}
}

peg::parser! {
	grammar parser() for str {
		rule digit() -> char = [c if c.is_ascii_digit()]
		rule number() -> u8 = c:$digit() { c.parse().unwrap() }

		pub rule bank() -> BatteryBank =
			ns:(number()+) { ns.into() }
	}
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 3;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let banks = parse(input,parser::bank);

		let total:usize = banks.map(|b| {

			let (at,d) = b.leftmost_max_val_at();

			let res = if at == b.len()-1 {
				// If it's in the last position,
				// Find _any_ max value among the rest
				(b.iter().copied().rev().skip(1).max().unwrap(),d)
			} else {
				// Find _any_ max value following it
				(d,b.iter().copied().skip(at+1).max().unwrap())
			};

			(res.0 * 10 + res.1) as usize

		}).sum();


		total
	}
}

#[cfg(test)]
mod test {

	use super::*;
	use rstest::*;

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		987654321111111
		811111111111119
		234234234234278
		818181911112111
		"#
	};

	#[test]
	fn test_parse() {

		let pack:Vec<BatteryBank> = parse(EXAMPLE_INPUT,parser::bank).collect();

		let actual_first_bank = &pack.first();
		let expected_first_bank = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];

		let actual_last_bank = &pack.last();
		let expected_last_bank = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];
	}

	#[test]
	fn test_part1_example() {
		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "357";
		assert_eq!(actual,expected);
	}


	#[rstest]
	#[case ("12345",(4,5))]
	#[case ("52345",(0,5))]
	#[case ("62345",(0,6))]
	fn test_bank_leftmost_max(
		#[case] bank_str:&str,
		#[case] max:(usize,u8)
	) {

		let bank = parser::bank(bank_str).unwrap();
		assert_eq!(bank.leftmost_max_val_at(),max);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		Part1::try_submit()?;
		Ok(())
	}
}
