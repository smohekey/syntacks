use std::future::Future;

use crate::{Error, Parsed, Source, SourceSpan};

pub trait Input<'src>: Sized + Clone + 'src {
	type Source: Source<'src>;
	type Item;

	fn source_span(&self) -> SourceSpan<'src, Self::Source>;
	fn next(self) -> Result<Parsed<'src, Self, Self::Item>, Error<'src, Self::Source>>;
	fn next_async(self) -> impl Future<Output = Result<Parsed<'src, Self, Self::Item>, Error<'src, Self::Source>>>;
}
