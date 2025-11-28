mod lexer;
mod parser;
mod ast;
mod modernism_ir;
mod spongelang_gen;
mod transpiler;

use std::fs;

fn main() {
    let code = fs::read_to_string("examples/acc_process.cobol")
        .expect("Cannot read COBOL file");

    let tokens = lexer::tokenize(&code);
    let ast = parser::parse(tokens);

    // AST → MODERNISM IR
    let modern_ir = transpiler::to_modernism_ir(ast);

    // MODERNISM IR → SPONGELANG IR
    let sponge_ir = spongelang_gen::modernism_to_spongelang(modern_ir);

    // SPONGELANG CODE
    let sponge_code = spongelang_gen::generate_spongelang(sponge_ir);

    println!("{}", sponge_code);
}

