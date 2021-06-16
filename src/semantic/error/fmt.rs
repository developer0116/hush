use std::fmt::Display as _;

use super::{Error, ErrorKind};
use crate::{
	fmt::Display,
	symbol,
};


impl<'a> Display<'a> for Error {
	type Context = &'a symbol::Interner;

	fn fmt(&self, f: &mut std::fmt::Formatter, context: Self::Context) -> std::fmt::Result {
		write!(f, "{} - ", self.pos)?;
		self.kind.fmt(f, context)
	}
}


impl<'a> Display<'a> for ErrorKind {
	type Context = &'a symbol::Interner;

	fn fmt(&self, f: &mut std::fmt::Formatter, context: Self::Context) -> std::fmt::Result {
		match self {
			Self::UndeclaredVariable(symbol) => {
				"undeclared variable '".fmt(f)?;
				symbol.fmt(f, context)?;
				"'".fmt(f)
			}

			Self::DuplicateVariable(symbol) => {
				"duplicate variable '".fmt(f)?;
				symbol.fmt(f, context)?;
				"'".fmt(f)
			}

			Self::DuplicateKey(symbol) => {
				"duplicate key '".fmt(f)?;
				symbol.fmt(f, context)?;
				"'".fmt(f)
			}

			Self::ReturnOutsideFunction => write!(f, "return statement outside function"),

			Self::SelfOutsideFunction => write!(f, "self variable outside function"),

			Self::BreakOutsideLoop => write!(f, "break statement outside loop"),

			Self::InvalidAssignment => write!(f, "invalid assignment"),

			Self::AsyncBuiltin => write!(f, "use of built-in command in async context"),
		}
	}
}


/// We need this in order to be able to implement std::error::Error.
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		Display::fmt(self, f, &symbol::Interner::new())
	}
}