use crate::ast::Node;

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
