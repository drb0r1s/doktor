use colored::Colorize;

pub fn get_prefix() -> String {
    format!("[{} Compiler]", "DOKTOR".bold()).magenta().to_string()
}