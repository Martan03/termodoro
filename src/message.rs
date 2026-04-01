#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    SplitSelect(usize),
    Continue,
    Finish,
}
