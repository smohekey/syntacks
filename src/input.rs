use std::future::Future;

use crate::{Error, Output, Source, SourceSpan};

pub trait Input<'src>: Sized + Clone + 'src {
	type Source: Source<'src>;
	type Item;

	fn source_span(&self) -> SourceSpan<'src, Self::Source>;
	fn next(self) -> Result<Output<'src, Self, Self::Item>, Error<'src, Self::Source>>;
	fn next_async(self) -> impl Future<Output = Result<Output<'src, Self, Self::Item>, Error<'src, Self::Source>>>;
}
