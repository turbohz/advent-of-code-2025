use std::io::repeat;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::Index;
use std::ops::Mul;
use std::ops::SubAssign;
use std::path::Iter;

use derive_more::{BitAnd,BitAndAssign};
use derive_more::{AsRef, Display, Index, IndexMut};
use heapless::deque::IntoIter;
use itertools::Itertools as _;
use funty::{AtMost16, Fundamental, Unsigned};
// use crate::days::day10::_day10::joltage;
use crate::days::util::AsBits;

use super::MachineDescription;
use super::{Inner, Bits, AsBits as _,Button};

// Represents a Joltage set.
//
// According to our analysis of the input, there are
// no more than 10 slots per machine, with values
// in the range 0..=273

const CAPACITY:usize = 10;

#[derive(Debug,Copy,Clone,PartialEq,Eq,AsRef)]
#[derive(Index,IndexMut,Display)]
#[display("{{{}}}",self.0.iter().take(self.len()).join(","))]
pub struct Joltage(
	#[index]#[index_mut]
	[u16;CAPACITY],
	u8,
);

impl Mul<Joltage> for u16 {
	type Output = Joltage;

	fn mul(self, jolt: Joltage) -> Self::Output {
		let vals:[u16;CAPACITY] = unsafe {
			jolt.0.into_iter()
			.map(|v| v*self)
			.collect_array()
			.unwrap_unchecked()
		};
		let len = jolt.1;
		Joltage(vals,len)
	}
}

impl AddAssign<Joltage> for Joltage {
	fn add_assign(&mut self, joltage: Joltage) {
		for i in 0..CAPACITY {
			self[i] = self[i] + joltage[i];
		}
	}
}

impl From<&MachineDescription> for Joltage {
	fn from(description: &MachineDescription) -> Self {
		description.joltage.iter().copied().map(|j| j as u16).into()
	}
}

impl AsBits<u16> for Joltage {
	fn as_bits(&self) -> Bits<u16> {
		use num::clamp;
		let bit_iter = self.0.into_iter().map(|v| v > 0);
		Bits::<u16>::from_iter(bit_iter)
	}
}

// No more than 10 values are to be provided.
// It they are, they will be ignored.
impl<T:Unsigned+AtMost16,I:IntoIterator<Item=T>> From<I> for Joltage where I::IntoIter: ExactSizeIterator {

	fn from(input: I) -> Self {
		use std::iter::repeat;

		let iter = input.into_iter();
		let len  = iter.len();

		let array = iter
			.map(Fundamental::as_u16)
			.chain(repeat(0))
			.take(CAPACITY)
			.collect_array()
			.unwrap();

		Self(array, len as u8)
	}
}

impl Joltage {

	#[inline]
   pub fn len(&self) -> usize {
      self.1 as usize
   }

	pub fn total(&self) -> usize {
		self.0.iter().sum::<u16>() as usize
	}

	pub fn as_mask(&self) -> Bits<u16> {
		self.as_bits()
	}

	pub fn zeroed(mut self) -> Self {
		self.0 = [0u16;CAPACITY];
		self
	}
}
