//! https://adventofcode.com/2025/day/3

use super::*;
use derive_more::{Deref, From};

#[derive(Debug)]
#[derive(From,Deref)]
struct BatteryBank(Vec<u8>);

impl BatteryBank {
	pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item=u8> {
		self.0.iter().copied()
	}
}

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

		let nums = self.max_of(2);

		(nums[0],nums[1])
	}

	fn max_of(&self,digits:usize) -> Vec<u8> {

		let mut res:Vec<u8> = Vec::with_capacity(digits);

		// Range will be adjusted as follows:
		// start: will be next position from last value found
		// end: starts with a reserve for N-1 digits,
		// increase limit as less values remain

		let mut range = 0..=self.len()-digits;

		while res.len() < digits {
			let (at,val) = first_max(&self[range.clone()]);
			res.push(val);
			range = range.start()+at+1..=range.end()+1
		}

		res
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

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 3;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> impl Display {

		let banks = parse(input,parser::bank);

		banks.map(|b| {

			let nums = b.max_of(12);
			// assemble digits
			nums.iter()
				.map(u8::to_string)
				.join("")
				.parse::<usize>()
				.unwrap()

		}).sum::<usize>()
	}
}

#[cfg(test)]
mod test {

	use super::*;

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

		use itertools::assert_equal;

		let pack:Vec<BatteryBank> = parse(EXAMPLE_INPUT,parser::bank).collect();

		let actual = pack.first().unwrap().iter();
		let expected = vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1];

		assert_equal(actual,expected);

		let actual = pack.last().unwrap().iter();
		let expected = vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1];

		assert_equal(actual,expected);
	}

	#[test]
	fn test_battery_bank() {

		// part 1

		let mut banks = parse(EXAMPLE_INPUT,parser::bank);

		assert_eq!((9,8),banks.next().unwrap().max_pair());
		assert_eq!((8,9),banks.next().unwrap().max_pair());
		assert_eq!((7,8),banks.next().unwrap().max_pair());
		assert_eq!((9,2),banks.next().unwrap().max_pair());

		// part 2

		let mut banks = parse(EXAMPLE_INPUT,parser::bank);

		assert_eq!(*parser::bank("987654321111").unwrap(),banks.next().unwrap().max_of(12));
		assert_eq!(*parser::bank("811111111119").unwrap(),banks.next().unwrap().max_of(12));
		assert_eq!(*parser::bank("434234234278").unwrap(),banks.next().unwrap().max_of(12));
		assert_eq!(*parser::bank("888911112111").unwrap(),banks.next().unwrap().max_of(12));
	}

	#[test]
	fn test_part1_example() {
		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "357";
		assert_eq!(actual,expected);
	}

	// SOLUTIONS

	submit! { Part1 }
	submit! { Part2 }
}
