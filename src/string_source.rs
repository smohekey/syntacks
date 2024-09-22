use crate::{Source, SourceSpan};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StringSource<'src> {
	text: &'src str,
}

impl<'src> StringSource<'src> {
	pub fn new(text: &'src str) -> Self {
		Self { text }
	}

	pub fn span(&self) -> SourceSpan<'src, Self> {
		SourceSpan::new(*self, 0, self.len())
	}

	pub fn bytes(&'src self) -> &'src [u8] {
		self.text.as_bytes()
	}

	pub fn byte_len(&self) -> usize {
		self.text.as_bytes().len()
	}
}

impl<'src> Source<'src> for StringSource<'src> {
	type Item = char;

	fn items(&'src self) -> impl Iterator<Item = Self::Item> {
		self.text.chars()
	}

	fn len(&self) -> usize {
		self.text.len()
	}

	fn is_empty(&self) -> bool {
		self.text.is_empty()
	}
}
