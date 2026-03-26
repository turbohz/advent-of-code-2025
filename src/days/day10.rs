// https://adventofcode.com/2025/day/10

use std::{fmt::Debug, ops::{BitXorAssign, Index, Not, SubAssign}};
use derive_more::{AsMut, AsRef, From};
use funty::{AtMost16, Fundamental, Unsigned};
use itertools::MinMaxResult;

use super::*;
use crate::days::util::AsBits as _;

#[derive(Debug,Default,AsRef,PartialEq,Eq)]
struct LightIndicators(Vec<bool>);

// Represents the light indicators state, as bits
// The size 16 has been chosen by assessing the inputs.
#[derive(Debug,Default,Clone,Copy,PartialEq,Eq,From,AsRef,AsMut)]
struct LightIndicatorsState(u16);

impl LightIndicatorsState {
	fn new(i: impl ExactSizeIterator<Item=bool> + Clone ) -> Self {

		let bits:Bits<u16> = Bits::try_from_iter(i)
			.expect("u16 should be able to contain all possible light indicators");

		Self(bits.into_inner())
	}
}

impl From<&LightIndicators> for LightIndicatorsState {
	fn from(value: &LightIndicators) -> Self {
		Self::new(value.as_ref().into_iter().copied())
	}
}

impl Display for LightIndicatorsState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let bools:Vec<bool> = self.as_bits().into_iter().collect();
		let str:String = bools.into_iter().map(|set| if set { '#' } else { '.' } ).collect();
		write!(f,"[{}]",str)
	}
}

impl BitXorAssign<Bits<u16>> for LightIndicatorsState {
	fn bitxor_assign(&mut self, rhs: Bits<u16>) {
		self.0 ^= rhs.into_inner()
	}
}

// By peeking at the input data, we see:
// There are no more than 16 joltage values per machine.
#[derive(Debug,Default,Copy,Clone,PartialEq,Eq,AsRef)]
struct Joltage([u16;16]);

// No more than 16 value are to be provided.
// It they are, they will be ignored.
impl<T:Unsigned+AtMost16,I:IntoIterator<Item=T>> From<I> for Joltage {
	fn from(input: I) -> Self {
		use std::iter::repeat;
		let iter = input.into_iter().map(Fundamental::as_u16).chain(repeat(0)).take(16);
		let array:[u16;16] = iter.collect_array().unwrap();
		Self(array)
	}
}

// Represents the flips the button performs, as a bitmask
#[derive(Clone,Copy,Default,PartialEq,Eq,Hash,AsRef,AsMut)]
struct Button(u16);

fn button(wires:&[u16]) -> Button {
	Button::new(wires.iter().copied())
}

impl Button {

	pub fn new<T:IntoIterator<Item=u16>>(input: T) -> Self {

		let inner = input
			.into_iter()
			.fold(0u16, |v,b| { v | 2u16.pow(b as u32) });
		Self(inner)
	}

	pub fn wire_count(&self) -> usize {
		self.as_ref().count_ones() as usize
	}

	pub fn wires(&self) -> impl Iterator<Item=usize> {
		self.as_bits()
			.into_iter()
			.digits()
			.positions(|v:u16| v > 0)
	}
}

impl Debug for Button {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Button").field(&format!("{:016b}",&self.0)).finish()
	}
}

impl Display for Button {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let wires = self.wires().join(",");
		write!(f,"({})",wires)
	}
}

#[derive(Debug,Clone,Copy)]
struct IndexedButtonSet<'desc> {
	buttons: &'desc Vec<Button>,
	enabled: Bits<u16>,
}

impl<'desc> Display for IndexedButtonSet<'desc> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"{}",self.buttons.iter().map(ToString::to_string).join(" "))
	}
}

impl<'desc> From<&'desc Vec<Button>> for IndexedButtonSet<'desc> {
	fn from(buttons: &'desc Vec<Button>) -> Self {
		use std::iter::repeat_n;
		let n = buttons.len();
		Self {
			buttons,
			enabled: Bits::try_from_iter(repeat_n(1u16,n)).unwrap()
		}
	}
}

impl<'desc> IndexedButtonSet<'desc> {

	pub fn len(&self) -> usize {
		self.buttons.len()
	}
}

impl<'desc> Index<usize> for IndexedButtonSet<'desc> {
	type Output = Button;

	fn index(&self, index: usize) -> &Self::Output {
		&self.buttons[index]
	}
}

#[derive(Debug,Default)]
struct MachineDescription {
	target: (LightIndicators,Joltage),
	buttons: Vec<Button>
}

impl MachineDescription {

	pub fn joltage(&self) -> &Joltage { &(self.target.1) }
	pub fn lights(&self)  -> &LightIndicators { &(self.target.0) }
	pub fn buttons_ref(&self) -> &Vec<Button> {
		&self.buttons
	}
}

#[derive(Debug,Clone)]
struct OperatingMachine<'desc,State> {
	target: State,
	state:  State,
	buttons: IndexedButtonSet<'desc>,
}

impl<'desc> From<&'desc MachineDescription> for OperatingMachine<'desc,LightIndicatorsState> {
	fn from(description: &'desc MachineDescription) -> Self {
		Self {
			target: description.lights().into(),
			state: Default::default(),
			buttons: description.buttons_ref().into(),
		}
	}
}

trait PressButton<State> {
	fn press(&mut self, idx:usize) -> State;
}

impl<'desc, State> OperatingMachine<'desc,State> {

	pub fn press_many(&mut self, is:impl Iterator<Item=usize>) -> State where State:Copy+Default, Self: PressButton<State> {
		is.map(|i| self.press(i)).last().unwrap_or_default()
	}

	pub fn reset(&mut self) where State:Default {
		self.state = Default::default();
	}

	pub fn jolted(&self)->bool where State:Eq {
		self.target == self.state
	}
}

impl<'desc> PressButton<LightIndicatorsState> for OperatingMachine<'desc,LightIndicatorsState> {
	fn press(&mut self, idx:usize) -> LightIndicatorsState {
		let btn = self.buttons[idx];
		self.state ^= btn.as_bits();
		self.state
	}
}

peg::parser! {

	grammar parser() for str {

		rule digit() -> char =
			[c if c.is_ascii_digit()]

		rule number() -> u16 =
			ds:$(digit()*<1,3>) {? ds.parse().or(Err("Expected usize value")) }

		rule light() -> bool = ['.'] { false } / ['#'] { true }

		rule lights() -> LightIndicators =
			"[" s:(light()+) "]" { LightIndicators(s) }

		rule button() -> Button =
			"(" bits:(number() ++ ",") ")" {
				Button::new(bits)
			}

		rule joltage() -> Joltage =
			"{" vals:(number() ++ ",") "}" {
				vals.into()
			}

		pub rule machine() -> MachineDescription =
			lights:lights() " " buttons:(button() ++ " ") " " joltage:joltage() {
				MachineDescription { target: (lights,joltage), buttons }
			}
	}
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 10;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> impl Display {

		let machines = parse(input, parser::machine);

		// Produce a generator of ALL _valuable_ sequences
		//
		// Button presses act as XOR operations on the machine state.
		// Given that XOR:
		// 	- cancels out if applied twice
		//    - is commutative and associative.
		// Then:
		// 	- 2,4, ... presses on a button do nothing
		//		- 1,3, ... presses on a button are all equivalent
		//
		// In other words:
		//  - Only single unique button presses do something meaninful without _extra_ effort.
		//  - Order of the button presses is irrelevant.

		let first_passing = |md:&MachineDescription| {

			let button_count:usize = md.buttons.len();
			let initial:OperatingMachine<LightIndicatorsState> = md.into();

			(0..button_count)
				// get unique button presses combinations, by increasing count
				.powerset()
				// first is the empty set
				.skip(1)
				// now find first machine that reaches the desired state
				.find(move |presses| {
					let mut machine = initial.clone();
					let final_state = machine.press_many(presses.iter().copied());
					final_state == machine.target
				})
		};

		machines
			.map(|ref md| first_passing(md).unwrap().len() )
			.sum::<usize>()
			.ok()
	}
}

/// A sorted sequence of button presses
struct ButtonPressSequence(Vec<u16>);

impl ButtonPressSequence {

	pub fn combinations(idxs:impl Iterator<Item=usize>, len:usize) -> impl Iterator<Item=Self> {
		idxs.map(|v| v as u16).combinations_with_replacement(len).map(ButtonPressSequence)
	}

	pub fn iter_idxs(&self) -> impl Iterator<Item=usize> {
		self.0.iter().copied().map(|v| v as usize)
	}

	pub fn counts(&self) -> [u16;16] {
		let counts = [0u16;16];

		self.iter_idxs().batching(|iter| {
			let idx = iter.next()?;
			let count = 1 + iter.take_while(|&v| v == idx).count() as u16;
			Some((idx,count))
		}).fold(counts,|mut counts,(idx,count)| {
			counts[idx] = count;
			counts
		})
	}
}
submit! { Part1 }

#[cfg(test)]
mod test {
	use super::*;
	use std::{default, vec};

	const EXAMPLE_INPUT:&str = indoc! {
		r#"
		[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
		[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
		[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
		"#
	};

	#[test]
	fn test_button() {

		let digits = [2,6,7];
		let actual = button(&digits);
		let expected = Button(0b11000100);

		assert_eq!(actual,expected);
		assert_eq!(actual.wire_count(),3);
		assert_equal(actual.wires(), [2,6,7]);
	}

	#[test]
	fn test_parse() {

		let input = "[.#..] (3) (1,3) (2) {3,5,4,7}";
		let machine = parser::machine(input).unwrap();

		let actual_target = machine.target;
		let expected_target = (LightIndicators(vec![false,true,false,false]), Joltage::from([3u16,5,4,7,0,0,0,0,0,0,0,0,0,0,0,0]));

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

	}

	#[test]
	fn test_example_part1() {

		let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
		let ref description:MachineDescription = parser::machine(input).unwrap();

		let mut machine:OperatingMachine<LightIndicatorsState> = description.into();

		// "You could press the first three buttons once each, a total of 3 button presses."

		machine.press_many(0..3);
		assert!(machine.jolted());

		// "You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4 button presses."
		// NOTICE: We don't really need the last two because pressing a button twice does nothing

		machine.reset();
		machine.press_many([1,3,5,5].into_iter());
		assert!(machine.jolted());

		// "You could press all of the buttons except (1,3) once each, a total of 5 button presses."

		machine.reset();
		machine.press_many([0,2,3,4,5].into_iter());
		assert!(machine.jolted());

		// "However, the fewest button presses required is 2.
		// One way to do this is by pressing the last two buttons ((0,2) and (0,1)) once each."

		machine.reset();
		machine.press_many([4,5].into_iter());
		assert!(machine.jolted());
	}

	#[test]
	fn test_part1() {

		let actual = Part1::solve(EXAMPLE_INPUT).unwrap().to_string();
		let expected = "7";

		assert_eq!(actual,expected);
	}

}
