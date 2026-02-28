#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn empty() -> Span {
        Span { line: 0, column: 0 }
    }
}
