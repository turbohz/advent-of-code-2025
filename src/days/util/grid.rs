use std::ops::{Index, IndexMut};
use super::*;

pub struct Grid<T> {
	pub size: Size,
	items: Vec<T>
}

impl<T> HasSize for Grid<T> {
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
