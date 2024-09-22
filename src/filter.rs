use std::marker::PhantomData;

use crate::{Error, Input, Parser, ParserResult};

pub fn filter<'src, I, O, P, F>(parser: P, predicate: F) -> Filter<'src, I, O, P, F>
where
	I: Input<'src>,
	P: Parser<'src, I, O>,
	F: FnMut(&O) -> bool,
{
	Filter::<'src, I, O, P, F> {
		parser,
		predicate,
		_phantom: PhantomData,
	}
}

pub struct Filter<'src, I, O, P, F>
where
	I: Input<'src>,
	P: Parser<'src, I, O>,
	F: FnMut(&O) -> bool,
{
	parser: P,
	predicate: F,
	_phantom: PhantomData<&'src (I, O)>,
}

impl<'src, I, O, P, F> Parser<'src, I, O> for Filter<'src, I, O, P, F>
where
	I: Input<'src>,
	P: Parser<'src, I, O>,
	F: FnMut(&O) -> bool,
	O: Clone,
{
	fn parse(&mut self, input: I) -> ParserResult<'src, I, O> {
		#[allow(clippy::manual_try_fold)]
		self
			.parser
			.parse(input)
			.iter()
			.filter(|output| (self.predicate)(&output.value))
			.cloned()
			.fold(Err(Error::NoMatch), |_, v| Ok(v))
	}

	async fn parse_async(&mut self, input: I) -> ParserResult<'src, I, O> {
		#[allow(clippy::manual_try_fold)]
		self
			.parser
			.parse_async(input)
			.await
			.iter()
			.filter(|output| (self.predicate)(&output.value))
			.cloned()
			.fold(Err(Error::NoMatch), |_, v| Ok(v))
	}
}
