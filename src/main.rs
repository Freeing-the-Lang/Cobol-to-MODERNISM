mod lexer;
mod parser;
mod ast;
mod modernism_ir;
mod spongelang_ir;
mod spongelang_gen;
mod transpiler;

use std::fs;

fn main() {
    let code = fs::read_to_string("examples/hello.cobol").unwrap();

    // 1) COBOL → AST
    let tokens = lexer::tokenize(&code);
    let ast = parser::parse(tokens);

    // 2) AST → Modernism IR
    let modern = transpiler::to_modernism_ir(ast);

    // 3) Modernism IR → Spongelang IR
    let sponge_ir = spongelang_gen::modernism_to_spongelang(modern);

    // 4) Spongelang 코드 생성
    let sponge_code = spongelang_gen::generate_spongelang(sponge_ir);

    println!("=== Spongelang OUTPUT ===");
    println!("{sponge_code}");
}
