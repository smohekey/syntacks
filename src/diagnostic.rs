use core::fmt;
use std::error::Error;

use crate::{Source, SourceSpan};

#[derive(Debug)]
pub enum Diagnostic<'src, S>
where
	S: Source<'src>,
{
	Message { source_span: SourceSpan<'src, S>, message: String },
	Error { source_span: SourceSpan<'src, S>, error: Box<dyn Error> },
}

impl<'src, S> Diagnostic<'src, S>
where
	S: Source<'src>,
{
	pub fn message(source_span: SourceSpan<'src, S>, message: impl Into<String>) -> Self {
		Self::Message {
			source_span,
			message: message.into(),
		}
	}

	pub fn error(source_span: SourceSpan<'src, S>, error: impl Error + 'static) -> Self {
		Self::Error {
			source_span,
			error: Box::new(error),
		}
	}
}

impl<'src, S> fmt::Display for Diagnostic<'src, S>
where
	S: Source<'src>,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO: fancy formatting and color

		match self {
			Diagnostic::Message { source_span, message } => todo!(),
			Diagnostic::Error { source_span, error } => write!(f, "{}", error),
		}
	}
}
