pub trait Inner<V:Copy>: AsRef<V> + Sized {
	fn as_inner(&self) -> V;
	fn into_inner(self) -> V;
}

pub trait InnerMut<V:Copy>: Inner<V> + AsMut<V> {
	fn inner_mut(&mut self) -> &mut V;
	fn map_inner<Mapper>(self,m:Mapper) -> Self where Mapper: Fn(V)->V;
	fn from(inner:V) -> Self where Self: Default;
}

impl<V:Copy,T:AsRef<V>> Inner<V> for T {

	fn as_inner(&self) -> V {
		self.as_ref().to_owned()
	}

	fn into_inner(self) -> V {
		(&self).as_inner()
	}
}

impl<V:Copy,T:Inner<V>+AsMut<V>> InnerMut<V> for T {

	fn inner_mut(&mut self) -> &mut V {
		self.as_mut()
	}

	fn map_inner<Mapper>(mut self, map_fn:Mapper) -> Self where Mapper: Fn(V)->V {
		*self.inner_mut() = map_fn(self.as_inner());
		self
	}

	fn from(inner:V) -> Self where Self: Default {
		let mut new = Self::default();
		*new.inner_mut() = inner;
		new
	}
}
