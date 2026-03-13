use std::{convert::identity, fmt::{Debug,Display}, ops::{Index, Not as _}};

use derive_more::{AsMut, AsRef, Display};
use funty::{AtMost32, Fundamental, Integral, Unsigned};
use itertools::{Itertools as _, repeat_n};

use crate::days::util::{AsBits, Inner as _};
use super::MachineDescription;
use super::{Bits,AsBits as _,joltage::Joltage};

// Represents a button: its wires an the jolt that provides.
//
// According to our analysis of the input, no button has
// more that 9 wires.
//
#[derive(Clone,Copy,PartialEq,Eq)]
#[derive(Display)]
#[display("({})",self.wires().into_iter().join(","))]
pub struct Button {
	wires: Bits<u16>,
	// the bump to joltage it causes (1 per wire)
	jolt: Joltage,
}

impl AsBits<u16> for Button {
	fn as_bits(&self) -> Bits<u16> {
		self.wires
	}
}

// Capacities derived from the problem input
pub type Wires = heapless::Vec<u8,9>;
pub type Buttons = heapless::Vec<Button,13>;

impl<Itm:Unsigned> FromIterator<Itm> for Button {
	fn from_iter<T: IntoIterator<Item = Itm>>(iter: T) -> Self {

		let wires:Bits<u16> = iter.into_iter()
			.fold(Bits(0),|mut bits,idx| bits.set(idx,true));
		let jolt:Joltage    = Joltage::from(wires.into_iter().digits::<u16>());

		Self { jolt, wires }
	}
}

impl Button {

	pub fn wires(&self) -> Wires {
		self.wires.iter()
			.positions(identity)
			.map(Fundamental::as_u8)
			.collect()
	}

	// find the maximum times the button can be pressed,
	// before overflowing the given joltage
	pub fn stride(&self, joltage:&Joltage) -> u16 {
		let values = self.wires().into_iter().map(|i| joltage[i as usize]);
		values.min().unwrap()
	}

	#[inline]
	pub fn is_wired_to<Idx:Integral>(&self,idx:Idx) -> bool {
		self.as_bits().get(idx)
	}
}

impl Debug for Button {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("Button").field(&format!("{:016b}",&self.wires.as_inner())).finish()
	}
}

// Creates a joltage value with only ones and zeroes.
// Used to generate a joltage delta from a button.
//
impl From<Button> for Joltage {
	fn from(button: Button) -> Self {
		Self::from(button.wires.into_iter().digits::<u16>())
	}
}

// Represents a set of buttons, and their usefulness to jolt.
// A button become useless when it would owerflow the joltage.
//
#[derive(Debug,Clone)]
pub struct ButtonList {
	buttons: Buttons,
	pub enabled: Bits<u16>,
}

impl Display for ButtonList {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

		let btns = self.iter().enumerate().map(|(i,b)| {

			if self.enabled(i) {
				b.to_string()
			} else {
				let mut under = b.wires().into_iter().map(|_|"_");
				format!("({})", under.join(","))
			}
		}).join(" ");

		write!(f,"{btns}")
	}
}

impl FromIterator<Button> for ButtonList {
	fn from_iter<T: IntoIterator<Item = Button>>(iter: T) -> Self {

		let buttons:Buttons   = iter.into_iter().collect();
		let enabled:Bits<u16> = <_>::from_iter(std::iter::repeat_n(1u16,buttons.len()));

		Self { buttons, enabled }
	}
}

impl From<&MachineDescription> for ButtonList {
	fn from(description: &MachineDescription) -> Self {

		let btns = description.buttons.iter()
			.map(|vals| Button::from_iter(vals.iter().copied()));

		Self::from_iter(btns)
	}
}

impl ButtonList {

	pub fn len(&self) -> usize {
		self.buttons.len()
	}

	pub fn enabled<Idx:Integral>(&self, i:Idx) -> bool {
		self.enabled.get(i)
	}

	pub fn enabled_idx(&self) -> impl Clone+Iterator<Item=usize> {
		self.enabled
			.into_iter()
			.positions(identity)
	}

	pub fn disable<Idx:Integral>(&mut self, i:Idx) {
		self.enabled.set(i,false);
	}

	pub fn iter(&self) -> impl Iterator<Item=&Button> {
		self.buttons.iter()
	}
}

impl Index<usize> for ButtonList {
	type Output = Button;

	fn index(&self, index: usize) -> &Self::Output {
		&self.buttons[index]
	}
}

#[cfg(test)]
mod test {

	use super::*;
	use itertools::assert_equal;

	pub fn button(input:&[i32]) -> Button {
		assert!(input.iter().all(|&i| i >= 0i32));
		Button::from_iter(input.iter().copied().map(Fundamental::as_u8))
	}

	#[test]
	fn test_button() {

		let btn = button(&[2,6,7]);

		assert_eq!(btn.jolt.to_string(),"{0,0,1,0,0,0,1,1,0,0}");
		assert_eq!(btn.wires,Bits(0b11000100));
		assert_equal(btn.wires(), [2,6,7]);

		let joltage:Joltage = [1u16,2,3].into();
		assert_eq!(btn.stride(&joltage),0);

		let btn = button(&[1,2]);
		assert_equal(btn.wires(), [1,2]);
		let joltage:Joltage = [1u16,2,3].into();
		assert_eq!(btn.stride(&joltage),2);
	}

	#[test]
	fn test_buttonset() {
		let buttons = vec![button(&[0,1]),button(&[10,12])];

		let mut buttonset = ButtonList::from_iter(buttons);
		assert!( buttonset.enabled_idx().contains(&0));
		assert!( buttonset.enabled_idx().contains(&1));

		buttonset.disable(0);
		assert!(!buttonset.enabled_idx().contains(&0));
		assert!( buttonset.enabled_idx().contains(&1));

		buttonset.disable(1);
		assert!(!buttonset.enabled_idx().contains(&0));
		assert!(!buttonset.enabled_idx().contains(&1));

	}
}
