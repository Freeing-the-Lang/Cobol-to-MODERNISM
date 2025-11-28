#[derive(Debug, Clone)]
pub enum SpongeIR {
    Block(String),
    Echo(String),
    Dissolve,
}
