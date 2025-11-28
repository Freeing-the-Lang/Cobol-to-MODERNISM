#[derive(Debug, Clone)]
pub enum ModernIR {
    Module(String),
    Print(String),
    Exit,
}
