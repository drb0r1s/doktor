use doktorc::frontend::tokenizer::Tokenizer;
use doktorc::frontend::parser::Parser;
use doktorc::frontend::resolver::Resolver;

use doktorc::backend::doktorb_writer::DoktorbWriter;

use doktorc::data::prefix::get_prefix;

use std::env;
use std::fs;
use std::process;

fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;

    let tokens = Tokenizer::new(&source).tokenize()?;
    let parser_doktor_node = Parser::new(tokens).parse()?;
    let (resolver_doktor_node, warnings, errors) = Resolver::new().resolve(parser_doktor_node);

    DoktorbWriter::write_doktorb(&resolver_doktor_node, "out/compiled.doktorb");

    println!("{} {} has been compiled to doktorc/out/compiled.doktorb.", get_prefix(), path);
    
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{} Usage: .\\doktorc.exe <[file_name].doktor>.", get_prefix());
        process::exit(1);
    }

    if let Err(e) = run(&args[1]) {
        eprintln!("{} {}.", get_prefix(), e);
        process::exit(1);
    }
}