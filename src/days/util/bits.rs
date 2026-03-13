use std::ops::RangeInclusive;

use super::*;

use derive_more::{AsMut, AsRef, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Display, From, Neg, Not};
use funty::{AtMost32, Fundamental, Integral, Numeric, Unsigned};
use num::cast::AsPrimitive;

#[derive(Default,Clone,Copy,PartialEq,Eq,From,AsRef,AsMut,Display)]
#[derive(BitAnd,BitAndAssign,BitOr,BitOrAssign,BitXor,BitXorAssign,Not,Neg)]
#[display("Bits({:01$b})",self.0,T::BITS as usize)]
pub struct Bits<T:Unsigned>(pub T);


impl<T:Unsigned> std::fmt::Debug for Bits<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self,f)
	}
}

pub trait AsBits<T:Unsigned> {
	fn as_bits(&self) -> Bits<T>;
}

impl<V:Unsigned,T:Inner<V>> AsBits<V> for T where Self: Sized {
	fn as_bits(&self) -> Bits<V> {
		Bits(self.into_inner())
	}
}

impl<Itm:Fundamental,T:Unsigned> FromIterator<Itm> for Bits<T> {

	/**
	 If the iterator is shorter that T::Bits, the value will be padded with zeroes.
	 If the iterator is longer than T::Bits, the extra value will be ignored.
	 */
	fn from_iter<Input: IntoIterator<Item=Itm>>(iter: Input) -> Self {

		use std::iter::repeat;

		let bits = iter
			.into_iter()
			// convert vals to 1 or 0, by comparing them with zero / false
			.map(|itm| { if itm.as_bool() { T::ONE } else { T::ZERO }})
			// pad with zeroess
			.pad_using(Self::LEN,|_|T::ZERO)
			// generate value for shifts
			.enumerate()
			// set bits ..
			.fold(T::default(),|acc:T,(shift,bit)|
				// take() makes shure we do not shl out of bounds
				acc | bit.wrapping_shl(shift as u32)
			);

		Bits(bits)
	}
}

#[derive(Debug,Clone,Copy)]
pub struct BitIter<T:Unsigned> {
	bits:T,
	down:u32
}

impl<T:Unsigned> Iterator for BitIter<T> {

	type Item = bool;

	fn next(&mut self) -> Option<Self::Item> {

		match self.down {
			0 => None,
			_ => {
				let bit = self.bits & T::ONE != T::ZERO;
				self.bits >>= 1;
				self.down -= 1;
				Some(bit)

			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.down as usize, Some(self.down as usize))
	}
}

impl<T:Unsigned> ExactSizeIterator for BitIter<T> {}

impl<T:Unsigned> From<Bits<T>> for BitIter<T> {
	fn from(value: Bits<T>) -> Self {
		BitIter {
			bits: value.into_inner(),
			down: T::BITS
		}
	}
}

impl<T:Unsigned> IntoIterator for Bits<T> {
	type Item = bool;

	type IntoIter = BitIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.into()
	}
}

impl<T:Unsigned> BitIter<T> {
	pub fn digits<I:Unsigned>(self) -> impl ExactSizeIterator<Item=I> where bool: AsPrimitive<I> {
		self.map(AsPrimitive::as_)
	}
}

impl<T:Unsigned> Bits<T> {

	const LEN:usize = { T::BITS as usize };

	pub fn iter(&self) -> BitIter<T> {
		self.into_iter()
	}

	/**
	Takes an iterator of values that can be interpreted as bool, and assembles
	a Bit instace.

	If the iterator is shorter that T::Bits, the value will be padded with zeroes.
	If the iterator is longer than T::Bits, the conversion will fail.
	*/
	pub fn try_from_iter<Item:Fundamental,Iter:ExactSizeIterator<Item=Item>>(iter:Iter) -> Option<Self> {

		if iter.len() > Self::LEN { return None }

		Some(Self::from_iter(iter))
	}

	#[inline]
	pub fn set<Idx:Integral>(&mut self, i:Idx, set:bool) -> Self {

		debug_assert!(i >= Idx::ZERO && i.as_usize() < Self::LEN);

		let bit = T::ONE.wrapping_shl(i.as_u32());

		if set { self.0 |= bit } else { self.0 &= !bit }
		*self
	}

	#[inline]
	pub fn get<Idx:Integral>(&self, i:Idx) -> bool {

		debug_assert!(i >= Idx::ZERO && i.as_usize() < Self::LEN);

		self.0 >> i.as_u32() & T::ONE == T::ONE
	}

}

#[cfg(test)]
mod test {

	use super::*;
	use itertools::assert_equal;

	const I:bool = true;
	const O:bool = false;

	#[test]
	fn test_bits() {

		assert_eq!(Bits(0x00FFu16).into_inner(),0x00FF);

		#[derive(AsRef)]
		struct Wrapped(u16);

		assert_eq!(Wrapped(0x00FFu16).as_bits(),Bits(0x00FFu16));

		let i = [1u8,1,0,0,1,1,0,0].into_iter();

		let expected = Some(Bits::<u8>(0b00110011));
		let actual = Bits::<u8>::try_from_iter(i);
		assert_eq!(actual,expected);

		// Can also use bools

		let i = [I,I,O,O,I,I,O,O].into_iter();

		let expected = Some(Bits::<u8>(0b00110011));
		let actual = Bits::<u8>::try_from_iter(i);
		assert_eq!(actual,expected);

	}

	#[test]
	fn test_into_iter() {

		let bits = Bits(0b01010011u8);

		assert_equal([1,1,0,0,1,0,1,0], bits.iter().digits::<u8>());
		assert_equal([I,I,O,O,I,O,I,O], bits);
	}

	fn test_set() {

		assert_eq!(Bits(0b01010011u8).set(2,true), Bits(0b01110011u8));
		assert_eq!(Bits(0b01010011u8).set(3,false), Bits(0b01000011u8));
	}
}
