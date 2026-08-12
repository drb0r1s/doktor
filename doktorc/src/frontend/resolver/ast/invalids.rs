use std::fmt;
use colored::Colorize;

use crate::data::prefix::get_prefix;

#[derive(Debug, Clone, PartialEq)]
pub struct ResolverWarning {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ResolverWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} [{}:{}]: {}.",
            get_prefix(), "(Resolver)".magenta().italic(), "Warning".on_yellow(), self.line, self.column, self.message
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResolverError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} [{}:{}]: {}.",
            get_prefix(), "(Resolver)".magenta().italic(), "Error".on_red(), self.line, self.column, self.message
        )
    }
}

impl std::error::Error for ResolverError {}