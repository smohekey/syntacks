pub trait Source<'src>: Clone + PartialEq {
	type Item;

	fn items(&'src self) -> impl Iterator<Item = Self::Item>;
	fn len(&self) -> usize;

	fn is_empty(&self) -> bool;
}
