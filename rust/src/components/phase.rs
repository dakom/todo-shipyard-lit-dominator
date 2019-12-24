#[derive(Eq, PartialEq)]
pub enum Phase {
    InitialLoad,
    Ready
}
impl Default for Phase {
    fn default() -> Self {
        Phase::InitialLoad 
    }
}