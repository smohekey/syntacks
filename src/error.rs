use std::{
	fmt::{self},
	str::Utf8Error,
};

use crate::{DiagnosticReport, Source};

#[derive(Debug, thiserror::Error)]
pub enum Error<'src, S>
where
	S: Source<'src>,
{
	NoMatch,
	EndOfInput,
	Utf8Error(#[from] Utf8Error),
	DiagnosticReport(DiagnosticReport<'src, S>),
}

impl<'src, S> fmt::Display for Error<'src, S>
where
	S: Source<'src> + fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
