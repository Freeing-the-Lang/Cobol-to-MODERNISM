use crate::modernism_ir::ModernIR;
use crate::spongelang_ir::SpongeIR;

pub fn modernism_to_spongelang(ir: Vec<ModernIR>) -> Vec<SpongeIR> {
    let mut out = vec![];

    for item in ir {
        match item {
            ModernIR::Module(name) => {
                out.push(SpongeIR::Block(name));
            },
            ModernIR::Print(msg) => {
                out.push(SpongeIR::Echo(msg));
            },
            ModernIR::Exit => {
                out.push(SpongeIR::Dissolve);
            }
        }
    }

    out
}
