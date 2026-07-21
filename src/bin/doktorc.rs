use doktor::frontend::tokenizer::Tokenizer;
use doktor::frontend::parser::Parser;
use doktor::frontend::resolver::Resolver;

use doktor::middleend::shaper::Shaper;
use doktor::middleend::painter::Painter;

use doktor::backend::packer::Packer;
use doktor::backend::doktorb_writer::DoktorbWriter;

use std::env;
use std::fs;
use std::process;
use std::path::Path;

fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;

    let tokens = Tokenizer::new(&source).tokenize()?;
    let doktor_node = Parser::new(tokens).parse()?;
    let (resolved_doktor_node, warnings, errors) = Resolver::new().resolve(doktor_node);

    let drawable_doktor_node = Shaper::new(1024.0, 1024.0).shape(resolved_doktor_node);
    let draw_structures = Painter::new().paint(drawable_doktor_node);

    let packed_packets = Packer::new().pack(&draw_structures);
    DoktorbWriter::write_doktorb(&packed_packets, Path::new("src/out/compiled.doktorb"));
    
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