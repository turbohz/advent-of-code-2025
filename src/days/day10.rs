// https://adventofcode.com/2025/day/10


use derive_more::{Deref, From, Into};

use super::*;

// Represents the machine state, as bits
// The size 16 has been chosen by assessing the inputs.
#[derive(Clone,Copy,Debug,Default,Into,PartialEq,Eq)]
struct MachineState(u16);

impl AsRef<u16> for MachineState {
	fn as_ref(&self) -> &u16 {
		&self.0
	}
}

impl<T:Clone+IntoIterator<Item=Option<()>>> From<T> for MachineState {
	fn from(options: T) -> Self {
		options
			.into_iter()
			.enumerate()
			.flat_map(|(i,op)| op.map(|_|i))
			.fold(MachineState::default(), |s,i| s.toggle_bit(i))
	}
}

impl MachineState {

	pub fn toggle<T:Into<u16>>(self,mask:T) -> Self {
		let inner:u16 = self.into();
		let xor_mask:u16 = mask.into();
		Self(inner ^ xor_mask)
	}

	pub fn toggle_bit(self,n:usize) -> Self {
		let mask = 2u16.pow(n as u32);
		self.toggle(mask)
	}
}

// Represents the flips the button performs, as a bitmask
#[derive(Debug,Clone,Copy,Default,PartialEq,Eq)]
struct Button(u16);

impl AsRef<u16> for Button {
	fn as_ref(&self) -> &u16 {
		&self.0
	}
}

impl<T:IntoIterator<Item=u8>> From<T> for Button {
	fn from(value: T) -> Self {
		let inner = value
			.into_iter()
			.fold(0u16, |v,b| { v | 2u16.pow(b as u32) });
		Self(inner)
	}
}

impl<V:Copy,T:AsRef<V>> IntoInner<V> for T {
	fn into_inner(self) -> V {
		*self.as_ref()
	}
}

#[derive(Debug,Default)]
struct MachineDescription {
	target: MachineState,
	buttons: Vec<Button>,
}

impl MachineDescription {
	fn state_iter<'a>(&'a self,is:impl IntoIterator<Item=usize>) -> MachineStateSequence<'a,impl Iterator<Item=usize>> {
		MachineStateSequence { buttons: &self.buttons, state: MachineState::default(), iter: is.into_iter() }
	}
}

struct MachineStateSequence<'a,I:Iterator<Item=usize>> {
	state: MachineState,
	buttons: &'a Vec<Button>,
	iter: I
}

impl<'a,I:Iterator<Item=usize>> Iterator for MachineStateSequence<'a,I> {

	type Item = MachineState;

	fn next(&mut self) -> Option<Self::Item> {
		let i = self.iter.next()?;
		let btn = self.buttons.get(i).expect("Requested button should exist.");
		let mask:u16 = btn.into_inner();
		self.state = self.state.toggle(mask);
		Some(self.state)
	}
}

peg::parser! {

	grammar parser() for str {

		rule digit() -> char =
			[c if c.is_ascii_digit()]

		rule number() -> usize =
			ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		rule light() -> Option<()> = ['.'] { None } / ['#'] { Some(()) }

		rule state() -> MachineState =
			"[" s:(light()+) "]" { s.into() }

		rule button() -> Button =
			"(" bits:(number() ++ ",") ")" {
				bits.into_iter().map(|n| n as u8).into()
			}

		pub rule machine() -> MachineDescription =
			target:state() " " buttons:(button() ++ " ") [_]* {
				MachineDescription { target, buttons }
			}
	}
}

fn find_shortest_activation_seq(m:MachineDescription)->impl Iterator<Item=usize> {

	// Button presses are just XOR operations on the machine state.
	// Given that XOR:
	// 	- cancels out if applied twice
	//    - is commutative and associative.
	// Then:
	// 	- 2,4, ... presses on a button do nothing
	//		- 1,3, ... presses on a button are all equivalent
	//
	// In other words, only single presses do something meaninful without extra effort.
   //
	// We just need a sequences of unique button presses in any order, sorted by length.

	(0..m.buttons.len())
		.powerset()
		// first is a []
		.skip(1)
		// ow, skip seqs that not achieve desired state
		.skip_while(|is| {
			// println!("{:?}",&is);
			m.state_iter(is.iter().copied()).last().unwrap() != m.target
		}).next().unwrap()
			.into_iter()
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 10;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let machines = parse(input, parser::machine);

		machines
			.map(|m| find_shortest_activation_seq(m).count() )
			.sum::<usize>()
	}
}

#[cfg(test)]
mod test {
	use std::vec;

use super::*;

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
		[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
		[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
		"#
	};

	#[test]
	fn test_state() {

		const INITIAL:MachineState = MachineState(0u16);

		assert_eq!(INITIAL.toggle_bit(0),MachineState(0b001));
		assert_eq!(INITIAL.toggle_bit(1),MachineState(0b010));
		assert_eq!(INITIAL.toggle_bit(2),MachineState(0b100));

		let bit_by_bit = INITIAL
			.toggle_bit(0)
			.toggle_bit(2);

		assert_eq!(bit_by_bit,MachineState(0b101));

		let reset = bit_by_bit.toggle(0b101 as u16);

		assert_eq!(reset,MachineState::default());
	}

	#[test]
	fn test_button() {

		let digits:Vec<u8> = vec![2,6,7];

		let actual:Button = digits.into();
		let expected:Button = Button(0b11000100);

		assert_eq!(actual,expected);
	}

	#[test]
	fn test_parse() {

		let input = "[.##.] (3) (1,3) (2) {3,5,4,7}";
		let machine = parser::machine(input).unwrap();

		let expected_target = MachineState(0b0110);
		let actual_target = machine.target;

		assert_eq!(actual_target,expected_target);

		let expected_buttons = vec![
			Button(0b1000), // 3
			Button(0b1010), // 3|1
			Button(0b0100), // 2
		];

		let actual_buttons = machine.buttons;

		assert_equal(actual_buttons, expected_buttons);
	}

	#[test]
	fn test_example() {

		let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
		let machine = parser::machine(input).unwrap();

		// "You could press the first three buttons once each, a total of 3 button presses."

		let seq = machine.state_iter(0..3);
		assert_eq!(seq.last().unwrap(), machine.target);

		// assert_err!(dbg!(machine.try_press_btn(0)));
		// assert_err!(dbg!(machine.try_press_btn(1)));
		// assert_ok!(dbg!(machine.try_press_btn(2)));

		// machine.reset();

		// assert_eq!(machine.current_state,MachineState(0));
		// assert_eq!(machine.target_state,MachineState(0b0110));

		// "You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4 button presses."
		// NOTICE: We don't really need the last two because pressing a button twice does nothing

		let seq = machine.state_iter([1,3,5,5]);
		assert_eq!(seq.last().unwrap(), machine.target);

		// "You could press all of the buttons except (1,3) once each, a total of 5 button presses."

		let seq = machine.state_iter([0,2,3,4,5]);
		assert_eq!(seq.last().unwrap(), machine.target);


		// "However, the fewest button presses required is 2.
		// One way to do this is by pressing the last two buttons ((0,2) and (0,1)) once each."

		let seq = machine.state_iter([4,5]);
		assert_eq!(seq.last().unwrap(), machine.target);
	}

	#[test]
	fn test_part1() {

		let actual = Part1::solve(EXAMPLE_INPUT).to_string();
		let expected = "7";

		assert_eq!(actual,expected);
	}

	// SOLUTIONS

	submit! { Part1 }

}
