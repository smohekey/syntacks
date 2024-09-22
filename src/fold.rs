use std::{
	iter::repeat,
	marker::PhantomData,
	ops::{Bound, RangeBounds},
};

use futures::stream::repeat as repeat_async;

use futures::StreamExt;

use crate::{async_utils::try_for_each, Error, Input, Parsed, Parser, SourceSpan};

pub fn fold<'src, I, O1, O2, P, R, FInit, FAccumulate>(
	parser: P,
	range: R,
	init: FInit,
	accumulate: FAccumulate,
) -> Fold<'src, I, O1, O2, P, R, FInit, FAccumulate>
where
	I: Input<'src>,
	P: Parser<'src, I, O1>,
	R: RangeBounds<usize>,
	FInit: FnMut() -> O2,
	FAccumulate: FnMut(&mut O2, O1),
{
	Fold::<'src, I, O1, O2, P, R, FInit, FAccumulate> {
		parser,
		range,
		init,
		accumulate,
		_phantom: PhantomData,
	}
}

pub struct Fold<'src, I, O1, O2, P, R, FInit, FAccumulate>
where
	I: Input<'src>,
	R: RangeBounds<usize>,
	P: Parser<'src, I, O1>,
	FInit: FnMut() -> O2,
	FAccumulate: FnMut(&mut O2, O1),
{
	parser: P,
	range: R,
	init: FInit,
	accumulate: FAccumulate,
	_phantom: PhantomData<&'src (I, O1)>,
}

struct State<'src, 'a, I, O1, O2, P, F>
where
	I: Input<'src>,
	P: Parser<'src, I, O1>,
	F: FnMut(&mut O2, O1),
{
	parser: &'a mut P,
	accumulate: &'a mut F,
	source_span: SourceSpan<'src, I::Source>,
	output: O2,
	remaining: I,
	_phantom: PhantomData<&'src O1>,
}

impl<'src, 'a, I, O1, O2, P, F> State<'src, 'a, I, O1, O2, P, F>
where
	I: Input<'src>,
	P: Parser<'src, I, O1>,
	F: FnMut(&mut O2, O1),
{
	fn parse(&mut self) -> Result<(), Error<'src, I::Source>> {
		let Parsed {
			source_span,
			output,
			remaining,
		} = self.parser.parse(self.remaining.clone())?;

		(self.accumulate)(&mut self.output, output);

		self.remaining = remaining;
		self.source_span += source_span;

		Ok::<_, Error<'src, I::Source>>(())
	}

	async fn parse_async(&mut self) -> Result<(), Error<'src, I::Source>> {
		let Parsed {
			source_span,
			output,
			remaining,
		} = self.parser.parse_async(self.remaining.clone()).await?;

		(self.accumulate)(&mut self.output, output);

		self.remaining = remaining;
		self.source_span += source_span;

		Ok::<_, Error<'src, I::Source>>(())
	}
}

impl<'src, I, O1, O2, P, R, FInit, FAccumulate> Parser<'src, I, O2> for Fold<'src, I, O1, O2, P, R, FInit, FAccumulate>
where
	I: Input<'src>,
	P: Parser<'src, I, O1>,
	R: RangeBounds<usize>,
	FInit: FnMut() -> O2,
	FAccumulate: FnMut(&mut O2, O1),
{
	fn parse(&mut self, input: I) -> crate::ParserResult<'src, I, O2> {
		let mut state: State<'src, '_, I, _, O2, P, FAccumulate> = State {
			parser: &mut self.parser,
			accumulate: &mut self.accumulate,
			source_span: input.source_span().start(),
			output: (self.init)(),
			remaining: input,
			_phantom: PhantomData::<&'src O1>,
		};

		repeat(()).take(range_to_minimum(&self.range)).try_for_each(|_| state.parse())?;
		repeat(()).take(range_to_maximum(&self.range)).try_for_each(|_| state.parse())?;

		Ok(Parsed {
			source_span: state.source_span,
			output: state.output,
			remaining: state.remaining,
		})
	}

	async fn parse_async(&mut self, input: I) -> crate::ParserResult<'src, I, O2> {
		let mut state: State<'src, '_, I, _, O2, P, FAccumulate> = State {
			parser: &mut self.parser,
			accumulate: &mut self.accumulate,
			source_span: input.source_span().start(),
			output: (self.init)(),
			remaining: input,
			_phantom: PhantomData::<&'src O1>,
		};

		try_for_each(repeat_async(()).take(range_to_minimum(&self.range)), |_| state.parse_async()).await?;
		try_for_each(repeat_async(()).take(range_to_maximum(&self.range)), |_| state.parse_async()).await?;

		Ok(Parsed {
			source_span: state.source_span,
			output: state.output,
			remaining: state.remaining,
		})
	}
}

fn range_to_minimum(range: &impl RangeBounds<usize>) -> usize {
	match range.start_bound() {
		Bound::Included(value) => *value,
		Bound::Excluded(value) => value + 1,
		Bound::Unbounded => 0,
	}
}

fn range_to_maximum(range: &impl RangeBounds<usize>) -> usize {
	match range.start_bound() {
		Bound::Included(value) => value + 1,
		Bound::Excluded(value) => *value,
		Bound::Unbounded => 0,
	}
}
