use crate::ast::Node;

#[derive(Debug, Clone)]
pub enum ModernIR {
    Module(String),
    Print(String),
    Exit,
}

pub fn ast_to_modernism_ir(ast: Vec<Node>) -> Vec<ModernIR> {
    let mut out = Vec::new();

    for node in ast {
        match node {
            Node::ProgramId(name) => {
                out.push(ModernIR::Module(name));
            }
            Node::Display(msg) => {
                out.push(ModernIR::Print(msg));
            }
            Node::StopRun => {
                out.push(ModernIR::Exit);
            }
        }
    }

    out
}
