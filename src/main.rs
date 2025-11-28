mod lexer;
mod parser;
mod ast;
mod transpiler;

use std::fs;

fn main() {
    let file_path = "examples/hello.cobol";
    let code = fs::read_to_string(file_path)
        .expect("Cannot read COBOL file.");

    println!("=== COBOL INPUT ===");
    println!("{code}");

    let tokens = lexer::tokenize(&code);
    let ast = parser::parse(tokens);
    let modern = transpiler::to_modernism(ast);

    println!("\n=== MODERN OUTPUT ===");
    println!("{modern}");
}
