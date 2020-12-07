use crate::piece_table::content::Source;

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub source: Source,
    pub position: usize,
    pub length: usize,
    pub line_breaks: Option<Vec<usize>>,
}

impl Span {
    pub fn new_sentinel() -> Self {
        Span {
            source: Source::Original,
            position: 0,
            length: 0,
            line_breaks: None,
        }
    }
}
