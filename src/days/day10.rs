// https://adventofcode.com/2025/day/10

/*

By running `analyze()` with the provided input,
we get these numbers:

Slots: 4..=10
Buttons: 2..=13
Wires: 1..=9
Jolt: 0..=273

*/

use std::{any, fmt::Debug};
use anyhow::{Context, bail};
use itertools::{MinMaxResult, repeat_n};

use super::*;
use crate::days::{day10::util::AsBits as _};

#[path="day10/parse.rs"]
mod parse;
use parse::{parser,MachineDescription};

#[path="day10/button.rs"]
mod button;
use button::{Button,ButtonList};

#[path="day10/lights.rs"]
mod lights;
use lights::LightIndicators;

#[path="day10/joltage.rs"]
mod joltage;
use joltage::Joltage;


#[derive(Debug,Clone)]
struct OperatingMachine<State> {
	target: State,
	state:  State,
	buttons: ButtonList,
}

impl From<&MachineDescription> for OperatingMachine<LightIndicators> {
	fn from(description: &MachineDescription) -> Self {
		Self {
			target: LightIndicators::from(description),
			state: Default::default(),
			buttons: ButtonList::from(description),
		}
	}
}

impl From<&MachineDescription> for OperatingMachine<Joltage> {
	fn from(description: &MachineDescription) -> Self {

		let target:Joltage = Joltage::from(description);
		let state:Joltage  = target.zeroed();
		let buttons = ButtonList::from(description);

		Self {
			target,
			state,
			buttons,
		}
	}
}

trait PressButton<State> {
	fn press(&mut self, idx:usize) -> State;
}

impl<State> OperatingMachine<State> {

	pub fn press_seq(&mut self, is:impl IntoIterator<Item=usize>) -> State where State:Copy, Self: PressButton<State> {
		is.into_iter().map(|i| self.press(i)).last().unwrap()
	}

	pub fn jolted(&self)->bool where State:Eq+Display {
		println!("{} == {}",self.target,self.state);
		self.target == self.state
	}
}

impl PressButton<LightIndicators> for OperatingMachine<LightIndicators> {
	fn press(&mut self, idx:usize) -> LightIndicators {
		let btn = self.buttons[idx];
		self.state ^= btn.as_bits();
		self.state
	}
}

impl OperatingMachine<Joltage> {

	fn press_many(&mut self, idx:usize, times: usize) -> Joltage {

		let button: Button = self.buttons[idx];
		let jolt = (times as u16) * Joltage::from(button);

		self.state += jolt;
		self.state
	}
}

impl PressButton<Joltage> for OperatingMachine<Joltage> {


	fn press(&mut self, idx:usize) -> Joltage {
		self.press_many(idx,1)
	}
}

// Analyze problem input, to figure out the ranges of values we're provided
fn analyze(input:&str) {

   use std::cmp::{self, min,max};

	let lines = input.lines();

	let machines = parse(input, parser::machine);

	let mut slots   = (usize::MAX,usize::MIN);
	let mut wires   = (usize::MAX,usize::MIN);
	let mut buttons = (usize::MAX,usize::MIN);
	let mut jolt    = (usize::MAX,usize::MIN);

	for (input,description) in lines.zip(machines) {

		println!("{}",input);

		// Slots

		{
			let count = description.lights.len();

			slots = (
				min(slots.0,count),
				max(slots.1,count)
			);
			println!("Slots: {:?}",slots.0..slots.1);
		}

		// Button

		{
			let count = description.buttons.len();
			buttons = (
				min(buttons.0,count),
				max(buttons.1,count),
			);
			println!("Buttons: {count}");
		}

		// Wires
		{
			let MinMaxResult::MinMax(min,max) = description.buttons.iter().map(|v| v.len()).minmax() else {
				unreachable!();
			};

			println!("Wires: {:?}",min..=max);

			wires = (
				cmp::min(wires.0,min),
				cmp::max(wires.1,max),
			)
		}

		// Jolt

		{
			let MinMaxResult::MinMax(min,max) = description.joltage.iter().copied().minmax() else {
				unreachable!()
			};

			jolt = (
				cmp::min(jolt.0,min),
				cmp::max(jolt.1,max),
			);
			println!("Jolt: {:?}", min..=max);
		}

	}

	println!("------------------------");
   println!("Slots: {:?}",slots.0..=slots.1);
	println!("Buttons: {:?}",buttons.0..=buttons.1);
	println!("Wires: {:?}",wires.0..=wires.1);
	println!("Jolt: {:?}",jolt.0..=jolt.1);
}

struct Part1;

impl Solution for Part1 {

	const DAY: i32 = 10;
	const PART: Part = Part::Part1;

	fn solve(input:&str) -> anyhow::Result<impl Display> {

		#[cfg(feature = "analyze")]
		{
			analyze(input);
			panic!()
		}

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
			let initial:OperatingMachine<LightIndicators> = md.into();

			(0..button_count)
				// get unique button presses combinations, by increasing count
				.powerset()
				// first is the empty set
				.skip(1)
				// now find first machine that reaches the desired state
				.find(move |presses| {
					let mut machine = initial.clone();
					let final_state = machine.press_seq(presses.iter().copied());
					final_state == machine.target
				})
		};

		machines
			.map(|ref md| first_passing(md).unwrap().len() )
			.sum::<usize>()
			.ok()
	}
}

struct Part2;

impl Solution for Part2 {

	const DAY: i32 = 10;
	const PART: Part = Part::Part2;

	fn solve(input:&str) -> anyhow::Result<impl Display> {

		use microlp::*;

		#[cfg(feature = "analyze")]
		{
			analyze(input);
			panic!()
		}

		let machines = parse(input, parser::machine);

		machines.map(|ref description| -> Result<usize,anyhow::Error> {

			use anyhow::Context;

			println!("{:?}",description);

			let machine:OperatingMachine<Joltage> = description.into();

			let slots = machine.target.len();
			let btns  = machine.buttons.len();

			// This solution solves the problem by modelling it as a system of linear
			// equations:
			// - the presses for each button are the variables
			// - then, for each slot we have a simple equation with the sum of the variables,
			//   and coefficients being 1.0 if the button is connection, and 0.0 if not.

			let mut problem = Problem::new(OptimizationDirection::Minimize);

			// Add vars (buttons in this case)
			// NOTICE: buttons can be pressed at most the minium of the joltage targets
			// among the slots they're wired to

			let mut variables = heapless::Vec::<Variable,13>::new();

			for btn_idx in 0..btns {
				let upper_bound = machine.buttons[btn_idx].stride(&machine.target).into();
				let variable = problem.add_integer_var(1.0, (0,upper_bound));
				unsafe { variables.push_unchecked(variable) };
			}

			// Add constraints
			// The sum of button presses for each slot must match its target value.

			for slot_idx in 0..slots {

				let target = machine.target[slot_idx];
				let mut expression = LinearExpr::empty();

				let coeff = |btn:&Button| -> f64 {
					if btn.is_wired_to(slot_idx) { 1.0 } else { 0.0 }
				};

				machine.buttons.iter()
					.enumerate()
					.for_each(|(idx,btn)|
						expression.add(variables[idx], coeff(btn))
					);

				problem.add_constraint(expression, ComparisonOp::Eq, target.into());
			}

			let total = problem.solve()?
				.iter()
				.map(|(_,val)| val.round() as usize)
				.sum();

			Ok(total)
		})
		.fold_ok(0,usize::saturating_add)
	}
}

submit! { Part1, Part2 }

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
	fn test_example_part1() {

		let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
		let ref description:MachineDescription = parser::machine(input).unwrap();

		// "You could press the first three buttons once each, a total of 3 button presses."

		let mut machine = OperatingMachine::<LightIndicators>::from(description);
		machine.press_seq(0..3);
		assert!(machine.jolted());

		// "You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4 button presses."
		// NOTICE: We don't really need the last two because pressing a button twice does nothing

		let mut machine = OperatingMachine::<LightIndicators>::from(description);
		machine.press_seq([1,3,5,5]);
		assert!(machine.jolted());

		// "You could press all of the buttons except (1,3) once each, a total of 5 button presses."

		let mut machine = OperatingMachine::<LightIndicators>::from(description);
		machine.press_seq([0,2,3,4,5]);
		assert!(machine.jolted());

		// "However, the fewest button presses required is 2.
		// One way to do this is by pressing the last two buttons ((0,2) and (0,1)) once each."

		let mut machine = OperatingMachine::<LightIndicators>::from(description);

		machine.press_seq([4,5]);
		assert!(machine.jolted());

	}

	#[test]
	fn test_machine_example_part2() {

		// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
		// Configuring the first machine's counters requires a minimum of 10 button presses.
		// One way to do this is by pressing:
		// - [0](3) once,
		// - [1](1,3) three times,
		// - [3](2,3) three times,
		// - [4](0,2) once,
		// - [5](0,1) twice.


		let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
		let ref description:MachineDescription = parser::machine(input).unwrap();
		let mut machine:OperatingMachine<Joltage> = description.into();
		machine.press_seq([0,1,1,1,3,3,3,4,5,5]);
		assert!(machine.jolted());

		// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
		// Configuring the second machine's counters requires a minimum of 12 button presses.
		// One way to do this is by pressing (0,2,3,4) twice, (2,3) five times, and (0,1,2) five times.

		let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
		let ref description:MachineDescription = parser::machine(input).unwrap();
		let mut machine:OperatingMachine<Joltage> = description.into();
		machine.press_seq([0,0,1,1,1,1,1,3,3,3,3,3]);
		assert!(machine.jolted());

		// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
		// Configuring the third machine's counters requires a minimum of 11 button presses.
		// One way to do this is by pressing (0,1,2,3,4) five times, (0,1,2,4,5) five times, and (1,2) once.

		let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
		let ref description:MachineDescription = parser::machine(input).unwrap();
		let mut machine:OperatingMachine<Joltage> = description.into();
		machine.press_seq([0,0,0,0,0,2,2,2,2,2,3]);
		assert!(machine.jolted());
	}

	#[test]
	fn test_part1() {

		let actual = Part1::solve(EXAMPLE_INPUT).unwrap().to_string();
		let expected = "7";

		assert_eq!(actual,expected);
	}

	#[test]
	fn test_part2() {

		let actual = Part2::solve(EXAMPLE_INPUT).unwrap().to_string();
		let expected = "33";

		assert_eq!(actual,expected);
	}
}
