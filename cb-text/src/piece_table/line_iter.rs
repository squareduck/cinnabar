use super::content::Source;
use super::span::Span;
use super::{PieceTable, SpanAddress};

pub struct LineIter<'pt> {
    piece_table: &'pt PieceTable,
    span_address: SpanAddress,
    line_break_index: usize,
    buffer: String,
}

impl<'pt> LineIter<'pt> {
    pub fn new(piece_table: &'pt PieceTable) -> Self {
        let span_address = SpanAddress {
            index: 0,
            offset: 0,
        };

        LineIter {
            piece_table,
            span_address,
            line_break_index: 0,
            buffer: String::new(),
        }
    }
}

impl<'pt> Iterator for LineIter<'pt> {
    type Item = String;

    fn nth(&mut self, n: usize) -> Option<String> {
        let mut skipped_line_breaks = 0;
        while skipped_line_breaks < n && self.span_address.index < self.piece_table.spans.len() {
            let span = &self.piece_table.spans[self.span_address.index];

            match &span.line_breaks {
                Some(line_breaks) if self.line_break_index > line_breaks.len() - 1 => {
                    self.span_address.index += 1;
                    self.span_address.offset = 0;
                    self.line_break_index = 0;
                }

                Some(line_breaks) => {
                    let line_break_position = line_breaks[self.line_break_index];
                    self.span_address.offset = line_break_position + 1;
                    self.line_break_index += 1;

                    skipped_line_breaks += 1;
                }
                None => {
                    self.span_address.index += 1;
                    self.span_address.offset = 0;
                    self.line_break_index = 0;
                }
            }
        }

        self.next()
    }

    fn next(&mut self) -> Option<String> {
        loop {
            if self.span_address.index > self.piece_table.spans.len() - 1 {
                if self.buffer.len() > 0 {
                    let line = self.buffer.clone();

                    self.buffer = String::new();

                    return Some(line);
                } else {
                    return None;
                }
            }

            let span = &self.piece_table.spans[self.span_address.index];

            match &span.line_breaks {
                Some(line_breaks) if self.line_break_index > line_breaks.len() - 1 => {
                    self.buffer.push_str(&self.piece_table.span_as_str(
                        span,
                        self.span_address.offset,
                        span.length,
                    ));

                    self.span_address.index += 1;
                    self.span_address.offset = 0;
                    self.line_break_index = 0;
                }
                Some(line_breaks) => {
                    let line_break_position = line_breaks[self.line_break_index];

                    self.buffer.push_str(&self.piece_table.span_as_str(
                        span,
                        self.span_address.offset,
                        line_break_position - self.span_address.offset,
                    ));

                    self.span_address.offset = line_break_position + 1;
                    self.line_break_index += 1;

                    let line = self.buffer.clone();

                    self.buffer = String::new();

                    return Some(line);
                }
                None => {
                    self.buffer.push_str(&self.piece_table.span_as_str(
                        span,
                        self.span_address.offset,
                        span.length,
                    ));

                    self.span_address.index += 1;
                    self.span_address.offset = 0;
                    self.line_break_index = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece_table::span::Span;
    use crate::piece_table::Change;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn iterates_over_lines() {
        let mut pt = PieceTable::from("\r\n067\n89\n\r\n");
        assert_eq!(pt.to_string(), "\r\n067\n89\n\r\n");

        let change = pt.insert(2, "1\r\n25\n");
        assert_eq!(pt.to_string(), "\r\n01\r\n25\n67\n89\n\r\n");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 2,
                        line_breaks: Some(vec![0])
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 5,
                        line_breaks: Some(vec![1, 4])
                    },
                    Span {
                        source: Source::Original,
                        position: 2,
                        length: 7,
                        line_breaks: Some(vec![2, 5, 6])
                    },
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 9,
                    line_breaks: Some(vec![0, 4, 7, 8])
                }]
            }
        );

        let change = pt.insert(5, "3\n4");
        assert_eq!(pt.to_string(), "\r\n01\r\n23\n45\n67\n89\n\r\n");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: Some(vec![1])
                    },
                    Span {
                        source: Source::Added,
                        position: 5,
                        length: 3,
                        line_breaks: Some(vec![1])
                    },
                    Span {
                        source: Source::Added,
                        position: 3,
                        length: 2,
                        line_breaks: Some(vec![1])
                    },
                ],
                old: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 5,
                    line_breaks: Some(vec![1, 4])
                }]
            }
        );

        let lines: Vec<String> = pt.lines().collect();

        assert_eq!(lines, vec!["", "01", "23", "45", "67", "89", ""]);
    }

    #[test]
    fn skips_over_lines() {
        let mut pt = PieceTable::from("0\n\n3\r\n4");
        pt.append("\n56\r\n78");
        pt.insert(2, "1\r\n2");

        let lines: Vec<String> = pt.lines().skip(4).collect();

        assert_eq!(lines, vec!["56", "78"]);
    }
}
