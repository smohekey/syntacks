use std::future::Future;

use crate::{Error, Input, Output};

pub type ParserResult<'src, I, O> = Result<Output<'src, I, O>, Error<'src, <I as Input<'src>>::Source>>;

pub trait Parser<'src, I, O>
where
	I: Input<'src>,
{
	fn parse(&mut self, input: I) -> ParserResult<'src, I, O>;
	fn parse_async(&mut self, input: I) -> impl Future<Output = ParserResult<'src, I, O>>;
}
