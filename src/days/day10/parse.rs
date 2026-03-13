use super::*;

#[derive(Debug,Default)]
pub struct MachineDescription {
	pub lights: Vec<bool>,
	pub buttons: Vec<Vec<usize>>,
	pub joltage: Vec<usize>,
}

peg::parser! {

	pub grammar parser() for str {

		rule digit() -> char =
			[c if c.is_ascii_digit()]

		rule number() -> usize =
			ds:$(digit()+) {? ds.parse().or(Err("Expected usize value")) }

		rule light() -> bool = ['.'] { false } / ['#'] { true }

		rule lights() -> Vec<bool> =
			"[" s:(light()+) "]" { s }

		rule button() -> Vec<usize> =
			"(" b:(number() ++ ",") ")" { b }

		rule joltage() -> Vec<usize> =
			"{" j:(number() ++ ",") "}" { j }

		pub rule machine() -> MachineDescription =
			lights:lights() " " buttons:(button() ++ " ") " " joltage:joltage() {
				MachineDescription { lights, joltage, buttons }
			}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {

		let input = "[.#..] (3) (1,3) (2) {3,5,4,7}";
		let machine = parser::machine(input).unwrap();

		let expected_lights = vec![false,true,false,false];
		let actual_lights = machine.lights;

		assert_eq!(actual_lights,expected_lights);

		let expected_buttons = vec![
			vec![3],
			vec![1,3],
			vec![2]
		];

		let actual_buttons = machine.buttons;

		assert_equal(actual_buttons, expected_buttons);
	}
}
