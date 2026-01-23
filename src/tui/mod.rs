pub mod active;
pub mod screen;
pub mod selector;
pub mod widgets;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IntervalType {
    Work,
    Pending(bool),
    Rest,
}

impl IntervalType {
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending(_))
    }
}
