use std::{fmt::Display, ops::BitXorAssign};
use derive_more::{AsMut, AsRef, Display, From};
use super::MachineDescription;
use super::{Bits,AsBits as _, Inner as _};

// Represents the light indicators state, as bits.
//
// According to our analysis of the input, there are
// no more than 10 light indicators per machine.
//
#[derive(Debug,Default,Clone,Copy,PartialEq,Eq)]
#[derive(From,AsRef,AsMut,Display)]
#[display("[{}]", self.as_bits().into_iter().map(|set| if set { '#' } else { '.' }).collect::<String>())]
pub struct LightIndicators(u16);

impl LightIndicators {
	pub fn new(i: impl ExactSizeIterator<Item=bool> + Clone ) -> Self {

		let bits:Bits<u16> = Bits::try_from_iter(i)
			.expect("u16 should be able to contain all possible light indicators");

		Self(bits.into_inner())
	}
}

impl BitXorAssign<Bits<u16>> for LightIndicators {
	fn bitxor_assign(&mut self, rhs: Bits<u16>) {
		self.0 ^= rhs.into_inner()
	}
}

impl From<&MachineDescription> for LightIndicators {
	fn from(description: &MachineDescription) -> Self {
		LightIndicators::new(description.lights.iter().copied())
	}
}
