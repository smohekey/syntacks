use crate::{Input, SourceSpan};

pub struct Output<'src, I, O>
where
	I: Input<'src>,
{
	pub source_span: SourceSpan<'src, I::Source>,
	pub value: O,
	pub remaining: I,
}

impl<'src, I, O> Clone for Output<'src, I, O>
where
	I: Input<'src>,
	O: Clone,
{
	fn clone(&self) -> Self {
		Self {
			source_span: self.source_span.clone(),
			value: self.value.clone(),
			remaining: self.remaining.clone(),
		}
	}
}
