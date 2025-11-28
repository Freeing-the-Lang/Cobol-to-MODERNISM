use crate::modernism_ir::ModernIR;

#[derive(Debug, Clone)]
pub enum SpongeIR {
    Block(String),
    Echo(String),
    Dissolve,
}

pub fn modernism_to_spongelang(ir: Vec<ModernIR>) -> Vec<SpongeIR> {
    let mut v = Vec::new();

    for item in ir {
        match item {
            ModernIR::Module(name) => v.push(SpongeIR::Block(name)),
            ModernIR::Print(msg) => v.push(SpongeIR::Echo(msg)),
            ModernIR::Exit => v.push(SpongeIR::Dissolve),
        }
    }

    v
}

pub fn generate_spongelang(ir: Vec<SpongeIR>) -> String {
    let mut out = String::new();

    for node in ir {
        match node {
            SpongeIR::Block(name) => {
                out.push_str(&format!("[sponge {}]\n", name));
            }
            SpongeIR::Echo(msg) => {
                out.push_str(&format!("  ~echo \"{}\"\n", msg));
            }
            SpongeIR::Dissolve => {
                out.push_str("  ~dissolve\n[/sponge]\n");
            }
        }
    }

    out
}
