use crate::ast::Node;
use crate::modernism_ir::{ModernIR, ast_to_modernism_ir};

pub fn to_modernism_ir(nodes: Vec<Node>) -> Vec<ModernIR> {
    ast_to_modernism_ir(nodes)
}

pub fn to_modernism(nodes: Vec<Node>) -> String {
    let mut out = String::new();

    for n in nodes {
        match n {
            Node::ProgramId(name) => {
                out.push_str(&format!("module {name} {{\n"));
            }
            Node::Display(msg) => {
                out.push_str(&format!("    println(\"{msg}\");\n"));
            }
            Node::StopRun => {
                out.push_str("    exit(0);\n}\n");
            }
        }
    }

    out
}
