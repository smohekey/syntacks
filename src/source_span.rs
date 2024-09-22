use std::{
	cmp::{max, min},
	marker::PhantomData,
	ops::{Add, AddAssign},
	str::{from_utf8, Utf8Error},
};

use crate::{Source, StringSource};

#[derive(Clone, Copy, Debug)]
pub struct SourceSpan<'src, S>
where
	S: Source<'src>,
{
	source: S,
	byte_start: usize,
	byte_end: usize,
	_phantom: PhantomData<&'src S>,
}

impl<'src, S> SourceSpan<'src, S>
where
	S: Source<'src>,
{
	pub fn new(source: S, byte_start: usize, byte_end: usize) -> Self {
		Self {
			source,
			byte_start,
			byte_end,
			_phantom: PhantomData,
		}
	}

	pub fn len(&self) -> usize {
		self.byte_end - self.byte_start
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn start(&self) -> Self {
		Self {
			source: self.source.clone(),
			byte_start: self.byte_start,
			byte_end: self.byte_start,
			_phantom: PhantomData,
		}
	}

	pub fn end(&self) -> Self {
		Self {
			source: self.source.clone(),
			byte_start: self.byte_end,
			byte_end: self.byte_end,
			_phantom: PhantomData,
		}
	}
}

impl<'src> SourceSpan<'src, StringSource<'src>> {
	pub fn as_str(&'src self) -> Result<&str, Utf8Error> {
		from_utf8(&self.source.bytes()[self.byte_start..self.byte_end])
	}
}

impl<'src, S> Add for SourceSpan<'src, S>
where
	S: Source<'src>,
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		assert!(self.source == rhs.source);

		Self {
			source: self.source,
			byte_start: min(self.byte_start, rhs.byte_start),
			byte_end: max(self.byte_end, rhs.byte_end),
			_phantom: PhantomData,
		}
	}
}

impl<'src, S> AddAssign for SourceSpan<'src, S>
where
	S: Source<'src>,
{
	fn add_assign(&mut self, rhs: Self) {
		assert!(self.source == rhs.source);

		self.byte_start = min(self.byte_start, rhs.byte_start);
		self.byte_end = max(self.byte_end, rhs.byte_end);
	}
}

#[cfg(test)]
mod test {
	use std::marker::PhantomData;

	use crate::StringSource;

	use super::SourceSpan;

	static SOURCE: &str = "some test source";

	fn source() -> StringSource<'static> {
		StringSource::new(SOURCE)
	}

	#[test]
	fn add_source_span() {
		let source = source();

		let span1 = SourceSpan {
			source,
			byte_start: 0,
			byte_end: 4,
			_phantom: PhantomData,
		};
		let span2 = SourceSpan {
			source,
			byte_start: 6,
			byte_end: 10,
			_phantom: PhantomData,
		};

		let span3 = span1 + span2;

		assert_eq!(0, span3.byte_start);
		assert_eq!(10, span3.byte_end);
		assert_eq!("some test ", span3.as_str().unwrap());
	}
}
