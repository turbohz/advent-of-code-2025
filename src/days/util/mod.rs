#![allow(dead_code)]

use super::*;


mod size;
pub use size::*;

mod location;
pub use location::*;

mod grid;
pub use grid::*;

mod inner;
pub use inner::*;

use std::ops::{Index, IndexMut};

use num::{Integer, integer::div_rem};

#[inline]
fn rem_div<T:Integer>(a:T,b:T) -> (T,T) {
	let (div,rem) = div_rem(a,b);
	(rem,div)
}
