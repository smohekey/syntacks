use std::str::from_utf8;

use crate::{Error, Input, Parsed, SourceSpan, StringSource};

#[derive(Clone, Copy, Debug)]
pub struct StringInput<'src> {
	source: StringSource<'src>,
	byte_start: usize,
	byte_end: usize,
}

impl<'src> StringInput<'src> {
	pub fn new(source: StringSource<'src>) -> Self {
		let byte_end = source.byte_len();

		Self {
			source,
			byte_start: 0,
			byte_end,
		}
	}
}

impl<'src> Input<'src> for StringInput<'src> {
	type Source = StringSource<'src>;
	type Item = char;

	fn source_span(&self) -> SourceSpan<'src, Self::Source> {
		SourceSpan::new(self.source, self.byte_start, self.byte_end)
	}

	fn next(self) -> Result<Parsed<'src, Self, Self::Item>, Error<'src, Self::Source>> {
		if self.byte_start <= self.byte_end {
			if let Some(bytes) = self.source.bytes().get(self.byte_start..self.byte_end) {
				return from_utf8(bytes).map_err(|error| error.into()).and_then(|string| {
					string
						.chars()
						.next()
						.map(|next| {
							let len = next.len_utf8();

							Parsed {
								output: next,
								source_span: SourceSpan::new(self.source, self.byte_start, self.byte_start + len),
								remaining: Self {
									source: self.source,
									byte_start: self.byte_start + len,
									byte_end: self.byte_end,
								},
							}
						})
						.ok_or_else(|| Error::EndOfInput)
				});
			}
		}

		Err(Error::EndOfInput)
	}

	async fn next_async(self) -> Result<Parsed<'src, Self, Self::Item>, Error<'src, Self::Source>> {
		self.next()
	}
}
