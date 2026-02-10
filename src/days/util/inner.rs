pub trait Inner<V>: AsRef<V>+AsMut<V> {
	fn as_inner(&self) -> V;
	fn into_inner(self) -> V;
	fn inner_mut(&mut self) ->&V;
}

impl<V:Copy,T:AsRef<V>+AsMut<V>> Inner<V> for T {

	fn as_inner(&self) -> V where Self: AsRef<V>, V:Copy{
		self.as_ref().to_owned()
	}

	fn into_inner(self) -> V where Self: Sized {
		self.as_inner()
	}

	fn inner_mut(&mut self) ->&V {
		self.as_mut()
	}
}
