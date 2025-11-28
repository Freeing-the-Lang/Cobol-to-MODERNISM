#[derive(Debug, Clone)]
pub enum Node {
    ProgramId(String),
    Display(String),
    StopRun,
}
