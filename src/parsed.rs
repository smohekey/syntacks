use crate::{Input, SourceSpan};

pub struct Parsed<'src, I, O>
where
	I: Input<'src>,
{
	pub source_span: SourceSpan<'src, I::Source>,
	pub output: O,
	pub remaining: I,
}

impl<'src, I, O> Clone for Parsed<'src, I, O>
where
	I: Input<'src>,
	O: Clone,
{
	fn clone(&self) -> Self {
		Self {
			source_span: self.source_span.clone(),
			output: self.output.clone(),
			remaining: self.remaining.clone(),
		}
	}
}
