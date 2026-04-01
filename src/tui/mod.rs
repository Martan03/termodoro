use crate::message::Message;

pub mod active;
pub mod overview;
pub mod screen;
pub mod selector;
pub mod widgets;

pub type Element = termint::widgets::Element<Message>;

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
