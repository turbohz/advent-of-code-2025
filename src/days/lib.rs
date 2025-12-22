use std::ops::{Index, IndexMut};

use super::*;

mod location {

	#[derive(Debug,Clone,Copy,PartialEq,Eq)]
	pub struct Location {
		pub x:usize,
		pub y:usize,
	}

	impl From<(usize,usize)> for Location {
		fn from((x,y): (usize,usize)) -> Self {
			Self { x, y }
		}
	}

	impl From<Location> for (usize,usize) {
			fn from(loc: Location) -> Self {
				(loc.x,loc.y)
			}
	}

	impl Location {

			pub fn down_unchecked(self) -> Self {
				Self { y: self.y+1, ..self }
			}

			pub fn up_unchecked(self) -> Self {
				Self { y: self.y-1, ..self }
			}

			pub fn right_unchecked(self) -> Self {
				Self { x: self.x+1, ..self }
			}

			pub fn left_unchecked(self) -> Self {
				Self { x: self.x-1, ..self }
			}
	}

}

pub use location::*;

mod size {

	pub trait HasSize {
		fn size(&self)->Size;
	}

	#[derive(Debug,Clone,Copy)]
	pub struct Size {
		pub width:usize,
		pub height:usize,
	}

	impl From<(usize,usize)> for Size {
		fn from(value: (usize,usize)) -> Self {
			Self { width: value.0, height: value.1 }
		}
	}
}

pub use size::*;

use derive_more::{Deref, Eq, From};
use num::{Integer, integer::div_rem};

#[inline]
fn rem_div<T:Integer>(a:T,b:T) -> (T,T) {
	let (div,rem) = div_rem(a,b);
	(rem,div)
}

pub struct Grid<T> {
	pub size: Size,
	items: Vec<T>
}

impl<T> size::HasSize for Grid<T> {
	fn size(&self)->Size {
		self.size
	}
}

impl<T> Grid<T> {

	pub fn new(width:usize,items:Vec<T>) -> Self {
		let size = (width, items.len() / width).into();
		Self { size, items }
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.items.len()
	}

	#[inline]
	pub fn stride(&self) -> usize {
		self.size.width
	}

	pub fn iter(&self) -> impl Iterator<Item=&T> {
		self.items.iter()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
		self.items.iter_mut()
	}

	pub fn enumerate(&self) -> impl Iterator<Item=((usize,usize),&T)> {
		self.items.iter().enumerate().map(|(idx,itm)| (i_to_xy(self.stride(), idx),itm))
	}
}

impl<I:Eq+PartialEq> Grid<I> {

	pub fn find_position(&self,item:I) -> Option<(usize,usize)> {
		self.items.iter()
			.find_position(|&i| *i == item)
			.map(|(i,_)| i_to_xy(self.stride(), i))
	}
}

#[inline]
fn xy_to_i(stride:usize, (x,y):(usize,usize)) -> usize {
	stride * y + x
}

#[inline]
fn i_to_xy(stride:usize, i:usize) -> (usize,usize) {
	rem_div(i, stride)
}

impl<T,I:Into<(usize,usize)>> Index<I> for Grid<T> {
	type Output = T;

	fn index(&self, index: I) -> &Self::Output {
		&self.items[xy_to_i(self.stride(),index.into())]
	}
}

impl<T,I:Into<(usize,usize)>> IndexMut<I> for Grid<T> {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		let stride = self.stride();
		self.items.get_mut(xy_to_i(stride,index.into())).unwrap()
	}
}
