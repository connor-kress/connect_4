#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Red,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
