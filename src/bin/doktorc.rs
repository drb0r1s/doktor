use doktor::frontend::tokenizer::Tokenizer;
use doktor::frontend::parser::Parser;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("[DOKTOR: Compiler] Usage: .\\doktorc.exe <source-code.doktor>");
        process::exit(1);
    }

    let path: &str = &args[1];

    let source: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[DOKTOR: Compiler] Could not open file '{}': {}", path, e);
            process::exit(1);
        }
    };

    let tokenizer: Tokenizer = Tokenizer::new(&source);

    match tokenizer.tokenize() {
        Ok(tokens) => {
            let parser: Parser = Parser::new(tokens);

            match parser.parse() {
                Ok(doktor_node) => {
                    println!("{}", doktor_node);
                }

                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }

        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}