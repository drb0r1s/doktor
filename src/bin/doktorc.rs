use doktor::frontend::tokenizer::Tokenizer;
use doktor::frontend::parser::Parser;
use doktor::frontend::resolver::Resolver;
use std::env;
use std::fs;
use std::process;

fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;

    let tokens = Tokenizer::new(&source).tokenize()?;
    let doktor_node = Parser::new(tokens).parse()?;
    let (resolved_doktor_node, warnings, errors) = Resolver::new().resolve(doktor_node);
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("[DOKTOR: Compiler] Usage: .\\doktorc.exe <source-code.doktor>");
        process::exit(1);
    }

    if let Err(e) = run(&args[1]) {
        eprintln!("[DOKTOR: Compiler] {}", e);
        process::exit(1);
    }
}