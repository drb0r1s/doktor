use crate::frontend::resolver::ast::invalids::ResolverWarning;

pub fn invalid_value_warning(name: &str, value: &str, line: usize, column: usize) -> ResolverWarning {
    ResolverWarning {
        message: format!(
            "\"{}\" has an invalid value \"{}\" and has been ignored",
            name, value
        ),
        line,
        column,
    }
}