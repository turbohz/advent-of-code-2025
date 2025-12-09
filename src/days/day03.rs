//! https://adventofcode.com/2025/day/3

use super::*;
use derive_more::{Deref, From};

#[derive(Debug)]
#[derive(From,Deref)]
struct BatteryBank(Vec<u8>);

// Returns the leftmost max value in a slice,
// along with its offset.
fn first_max(slice:&[u8])->(usize,u8) {

	slice.iter().copied().enumerate()
		.max_set_by(|(_,v1),(_,v2)| v1.cmp(v2))
		.first()
		.unwrap()
		.to_owned()
	}

impl BatteryBank {

	fn max_pair(&self) -> (u8,u8) {

		// look for max value, excluding last
		let ref fst_haystack = self[..self.len()-1];

		let (fst_at,fst) = first_max(fst_haystack);

		// look for max value among the following values
		let ref snd_haystack = self[fst_at+1..];

		let (_,snd) = first_max(snd_haystack);

		(fst,snd)
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

		banks.map(|b| {
			let (fst,snd) = b.max_pair();
			(fst*10+snd) as usize
		}).sum::<usize>()
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
	fn test_battery_bank() {

		let mut banks = parse(EXAMPLE_INPUT,parser::bank);

		assert_eq!((9,8),banks.next().unwrap().max_pair());
		assert_eq!((8,9),banks.next().unwrap().max_pair());
		assert_eq!((7,8),banks.next().unwrap().max_pair());
		assert_eq!((9,2),banks.next().unwrap().max_pair());
	}

	#[test]
	fn test_part1_example() {
		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "357";
		assert_eq!(actual,expected);
	}

	#[test]
	fn submit()-> Result<(), AppError> {
		Part1::try_submit()?;
		Ok(())
	}
}
