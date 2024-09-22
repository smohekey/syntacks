use std::{marker::PhantomData, ops::RangeBounds};

use crate::{filter, fold, Input, Parser, ParserResult};

pub fn char<'src, I>() -> Char<'src, I>
where
	I: Input<'src, Item = char>,
{
	Char::<'src, I>(PhantomData)
}

pub struct Char<'src, I>(PhantomData<&'src I>)
where
	I: Input<'src, Item = char>;

impl<'src, I> Parser<'src, I, char> for Char<'src, I>
where
	I: Input<'src, Item = char>,
{
	fn parse(&mut self, input: I) -> ParserResult<'src, I, char> {
		input.next()
	}

	async fn parse_async(&mut self, input: I) -> ParserResult<'src, I, char> {
		input.next_async().await
	}
}

pub fn alpha<'src, I>(range: impl RangeBounds<usize>) -> impl Parser<'src, I, ()>
where
	I: Input<'src, Item = char>,
{
	fold(filter(char(), |c| c.is_ascii_alphabetic()), range, || (), |_, _| ())
}

pub fn alpha0<'src, I>() -> impl Parser<'src, I, ()>
where
	I: Input<'src, Item = char>,
{
	alpha(0..)
}

pub fn alpha1<'src, I>() -> impl Parser<'src, I, ()>
where
	I: Input<'src, Item = char>,
{
	alpha(1..)
}

pub fn alphanum<'src, I>(range: impl RangeBounds<usize>) -> impl Parser<'src, I, ()>
where
	I: Input<'src, Item = char>,
{
	fold(filter(char(), |c| c.is_ascii_alphanumeric()), range, || (), |_, _| ())
}

pub fn alphanum0<'src, I>() -> impl Parser<'src, I, ()>
where
	I: Input<'src, Item = char>,
{
	alphanum(0..)
}

pub fn alphanum1<'src, I>() -> impl Parser<'src, I, ()>
where
	I: Input<'src, Item = char>,
{
	alphanum(1..)
}
