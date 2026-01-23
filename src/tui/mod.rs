pub mod active;
pub mod screen;
pub mod selector;
pub mod widgets;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IntervalType {
    Work,
    Rest,
}
