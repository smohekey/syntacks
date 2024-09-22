use core::{error::Error, fmt};

use crate::{Diagnostic, Source};

#[derive(Debug)]
pub struct DiagnosticReport<'src, S>
where
	S: Source<'src>,
{
	diagnostics: Vec<Diagnostic<'src, S>>,
}

impl<'src, S> From<Diagnostic<'src, S>> for DiagnosticReport<'src, S>
where
	S: Source<'src>,
{
	fn from(value: Diagnostic<'src, S>) -> Self {
		Self { diagnostics: vec![value] }
	}
}

impl<'src, S> fmt::Display for DiagnosticReport<'src, S>
where
	S: Source<'src>,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for diagnostic in self.diagnostics.iter() {
			write!(f, "{}", diagnostic)?;
		}

		Ok(())
	}
}

impl<'src, S> Error for DiagnosticReport<'src, S>
where
	S: Source<'src> + fmt::Debug,
{
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		None
	}

	fn description(&self) -> &str {
		"description() is deprecated; use Display"
	}

	fn cause(&self) -> Option<&dyn Error> {
		self.source()
	}
}
