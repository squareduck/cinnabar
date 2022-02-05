mod content;
mod line_iter;
mod match_iter;
mod span;

use regex::Regex;
use std::fmt;

use self::content::{Content, Source};
use self::line_iter::LineIter;
use self::match_iter::MatchIter;
use self::span::Span;

/// Position in text in terms of span index and offset in that span.
#[derive(Debug, Clone, PartialEq)]
pub struct SpanAddress {
    index: usize,
    offset: usize,
}

/// Reversible change of spans.
/// Applying a change replaces old spans at indicated index with new spans.
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    index: usize,
    new: Vec<Span>,
    old: Vec<Span>,
}

pub struct PieceTable {
    original: Content,
    added: Content,
    spans: Vec<Span>,
    extend_spans: bool,
}

impl<'pt> PieceTable {
    fn new(content: String) -> Self {
        let mut original_buffer = Content::new(Source::Original);
        let original_span = original_buffer.append(&content);

        let added_buffer = Content::new(Source::Added);

        let sentinel_span = Span::new_sentinel();

        let spans = vec![original_span, sentinel_span];

        PieceTable {
            original: original_buffer,
            added: added_buffer,
            spans,
            extend_spans: false,
        }
    }

    fn span_as_str(&'pt self, span: &Span, offset: usize, limit: usize) -> &'pt str {
        let buffer = match span.source {
            Source::Original => self.original.content_for_span(span, offset, limit),
            Source::Added => self.added.content_for_span(span, offset, limit),
        };

        buffer
    }

    fn range_as_string(&'pt self, position: usize, length: usize) -> String {
        let start_span_address = self.span_address(position);
        let end_span_address = self.span_address(position + length);

        let total_span_count = end_span_address.index - start_span_address.index + 1;

        self.spans
            .iter()
            .skip(start_span_address.index)
            .take(total_span_count)
            .enumerate()
            .map(|(relative_index, span)| {
                let offset = if relative_index == 0 {
                    start_span_address.offset
                } else {
                    0
                };

                let limit = if relative_index == total_span_count - 1 {
                    end_span_address.offset
                } else {
                    span.length
                };

                self.span_as_str(&span, offset, limit)
            })
            .fold(String::new(), |mut acc, s| {
                acc.push_str(s);
                acc
            })
    }

    /// Returns a span index and offset that corresponds to position in text
    fn span_address(&self, position: usize) -> SpanAddress {
        let mut start_span_position = 0;

        for (index, span) in self.spans.iter().enumerate() {
            let end_span_position = start_span_position + span.length;

            if position >= start_span_position && position < end_span_position {
                return SpanAddress {
                    index,
                    offset: position - start_span_position,
                };
            }

            start_span_position += span.length;
        }

        // If span is not found returns sentinel span at the end
        SpanAddress {
            index: self.spans.len() - 1,
            offset: 0,
        }
    }

    fn span_content(&self, span: &Span) -> &Content {
        match span.source {
            Source::Original => &self.original,
            Source::Added => &self.added,
        }
    }

    fn extend_span(
        &mut self,
        start_address: &SpanAddress,
        end_address: &SpanAddress,
        content: &str,
    ) -> Option<Change> {
        if self.extend_spans == true
            && start_address.index > 0
            && start_address.offset == 0
            && start_address == end_address
            && self.spans[start_address.index - 1].source == Source::Added
        {
            let previous_span = &self.spans[start_address.index - 1];
            let mut new_span = self.added.append(content);
            new_span.position = previous_span.position;
            new_span.length += previous_span.length;

            Some(Change {
                index: start_address.index - 1,
                new: vec![new_span],
                old: vec![previous_span.clone()],
            })
        } else {
            None
        }
    }

    fn change(&mut self, position: usize, length: usize, content: &str) -> Change {
        let start_address = self.span_address(position);
        let end_address = if length == 0 {
            start_address.clone()
        } else {
            self.span_address(position + length)
        };

        if let Some(change) = self.extend_span(&start_address, &end_address, content) {
            return change;
        }

        let new_span = if content.len() > 0 {
            Some(self.added.append(content))
        } else {
            None
        };

        let new_start_span = if start_address.offset > 0 {
            let start_span = self.spans[start_address.index].clone();
            let start_span_length = start_address.offset;

            let line_breaks = match start_span.line_breaks {
                Some(line_breaks) => {
                    let new_line_breaks: Vec<usize> = line_breaks
                        .into_iter()
                        .filter(|lbr| lbr < &start_span_length)
                        .collect();

                    if new_line_breaks.len() > 0 {
                        Some(new_line_breaks)
                    } else {
                        None
                    }
                }
                None => None,
            };

            Some(Span {
                source: start_span.source,
                position: start_span.position,
                length: start_span_length,
                line_breaks,
            })
        } else {
            None
        };

        let new_end_span = if end_address.offset > 0 {
            let end_span = self.spans[end_address.index].clone();
            let end_span_length = end_span.length - end_address.offset;

            let line_breaks = match end_span.line_breaks {
                Some(line_breaks) => {
                    let new_line_breaks: Vec<usize> = line_breaks
                        .into_iter()
                        .filter(|lbr| lbr > &end_address.offset)
                        .map(|lbr| lbr - end_address.offset)
                        .collect();

                    if new_line_breaks.len() > 0 {
                        Some(new_line_breaks)
                    } else {
                        None
                    }
                }
                None => None,
            };

            Some(Span {
                source: end_span.source,
                position: end_span.position + end_address.offset,
                length: end_span_length,
                line_breaks,
            })
        } else {
            None
        };

        let splice_range = match new_end_span {
            Some(_) => start_address.index..end_address.index + 1,
            None => start_address.index..end_address.index,
        };

        let mut inserted_spans = vec![];

        if let Some(span) = new_start_span {
            inserted_spans.push(span);
        }

        if let Some(span) = new_span {
            inserted_spans.push(span);
        }

        if let Some(span) = new_end_span {
            inserted_spans.push(span);
        }

        let removed_spans: Vec<Span> = self.spans[splice_range]
            .iter()
            .map(|span| span.clone())
            .collect();

        Change {
            index: start_address.index,
            new: inserted_spans,
            old: removed_spans,
        }
    }

    pub fn apply_change(&mut self, change: &Change) {
        self.spans.splice(
            change.index..change.index + change.old.len(),
            change.new.clone(),
        );
    }

    pub fn revert_change(&mut self, change: &Change) {
        self.spans.splice(
            change.index..change.index + change.new.len(),
            change.old.clone(),
        );
    }

    pub fn replace(&mut self, position: usize, length: usize, content: &str) -> Change {
        let change = self.change(position, length, content);

        self.apply_change(&change);
        change
    }

    pub fn append(&mut self, content: &str) -> Change {
        let position = self.spans.iter().map(|span| span.length).sum::<usize>();
        self.replace(position, 0, content)
    }

    pub fn insert(&mut self, position: usize, content: &str) -> Change {
        self.replace(position, 0, content)
    }

    pub fn remove(&mut self, position: usize, length: usize) -> Change {
        self.replace(position, length, "")
    }

    pub fn find_matches(&self, position: usize, regex: Regex) -> MatchIter {
        MatchIter::new(self, position, regex)
    }

    pub fn lines(&self) -> LineIter {
        LineIter::new(self)
    }
}

impl<T> From<T> for PieceTable
where
    T: Into<String>,
{
    fn from(item: T) -> Self {
        let content: String = item.into();

        PieceTable::new(content)
    }
}

impl fmt::Display for PieceTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span in self.spans.iter() {
            write!(f, "{}", self.span_as_str(span, 0, span.length))?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq, assert_ne};

    fn print_buffers(piece_table: &PieceTable) {
        println!("O: {}", piece_table.original.buffer);
        println!("A: {}", piece_table.added.buffer);
    }

    fn print_spans(piece_table: &PieceTable) {
        for span in piece_table.spans.iter() {
            let source = match span.source {
                Source::Original => "O",
                Source::Added => "A",
            };

            print!(
                "[{}: {}..{}, ",
                source,
                span.position,
                span.position + span.length
            );
            print!("{}]\n", piece_table.span_as_str(&span, 0, span.length));
        }
    }

    // # Initialization

    #[test]
    fn creates_piece_table_from_str() {
        let pt = PieceTable::from("012");
        assert_eq!(pt.to_string(), "012");
    }

    // # Appends

    #[test]
    fn appends() {
        let mut pt = PieceTable::from("012");

        let change = pt.append("345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.append("678");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 2,
                new: vec![Span {
                    source: Source::Added,
                    position: 3,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn appends_with_extend_spans() {
        let mut pt = PieceTable::from("012");
        pt.extend_spans = true;

        let change = pt.append("345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.append("678");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 6,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }]
            }
        );
    }

    // # Inserts

    #[test]
    fn inserts_at_the_start() {
        let mut pt = PieceTable::from("678");

        let change = pt.insert(0, "345");
        assert_eq!(pt.to_string(), "345678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.insert(0, "012");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Added,
                    position: 3,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn inserts_at_the_start_with_extend_spans() {
        let mut pt = PieceTable::from("678");
        pt.extend_spans = true;

        let change = pt.insert(0, "345");
        assert_eq!(pt.to_string(), "345678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.insert(0, "012");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Added,
                    position: 3,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn inserts_at_the_end() {
        let mut pt = PieceTable::from("012");

        let change = pt.insert(3, "345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.insert(6, "678");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 2,
                new: vec![Span {
                    source: Source::Added,
                    position: 3,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn inserts_at_the_end_with_extend_spans() {
        let mut pt = PieceTable::from("012");
        pt.extend_spans = true;

        let change = pt.insert(3, "345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.insert(6, "678");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 6,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn inserts_at_position_beyond_end() {
        let mut pt = PieceTable::from("012");

        let change = pt.insert(6, "345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.insert(9, "678");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 2,
                new: vec![Span {
                    source: Source::Added,
                    position: 3,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn inserts_at_span_boundary() {
        let mut pt = PieceTable::from("012");

        let change = pt.insert(3, "678");
        assert_eq!(pt.to_string(), "012678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );

        let change = pt.insert(3, "345");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 3,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn inserts_inside_span() {
        let mut pt = PieceTable::from("036");

        let change = pt.insert(1, "12");
        assert_eq!(pt.to_string(), "01236");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 1,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 2,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Original,
                        position: 1,
                        length: 2,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }]
            }
        );

        let change = pt.insert(4, "45");
        assert_eq!(pt.to_string(), "0123456");
        assert_eq!(
            change,
            Change {
                index: 2,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 1,
                        length: 1,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 2,
                        length: 2,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Original,
                        position: 2,
                        length: 1,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 1,
                    length: 2,
                    line_breaks: None
                }]
            }
        );

        let change = pt.insert(7, "78");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 5,
                new: vec![Span {
                    source: Source::Added,
                    position: 4,
                    length: 2,
                    line_breaks: None
                },],
                old: vec![]
            }
        );
    }

    // # Removes

    #[test]
    fn removes_at_the_start() {
        let mut pt = PieceTable::from("012345678");

        let change = pt.remove(0, 3);
        assert_eq!(pt.to_string(), "345678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Original,
                    position: 3,
                    length: 6,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 9,
                    line_breaks: None
                }]
            }
        );

        let change = pt.remove(0, 3);
        assert_eq!(pt.to_string(), "678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Original,
                    position: 6,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Original,
                    position: 3,
                    length: 6,
                    line_breaks: None
                }]
            }
        );

        let change = pt.remove(0, 3);
        assert_eq!(pt.to_string(), "");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![],
                old: vec![Span {
                    source: Source::Original,
                    position: 6,
                    length: 3,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn removes_at_the_end() {
        let mut pt = PieceTable::from("012345678");

        let change = pt.remove(6, 3);
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 6,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 9,
                    line_breaks: None
                }]
            }
        );

        let change = pt.remove(3, 3);
        assert_eq!(pt.to_string(), "012");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 6,
                    line_breaks: None
                }]
            }
        );

        let change = pt.remove(0, 3);
        assert_eq!(pt.to_string(), "");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn removes_at_position_beyond_end() {
        let mut pt = PieceTable::from("012345678");

        let change = pt.remove(9, 3);
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![],
                old: vec![]
            }
        );
    }

    #[test]
    fn removes_with_length_beyond_end() {
        let mut pt = PieceTable::from("012345678");

        let change = pt.remove(6, 6);
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 6,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 9,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn removes_at_span_boundary() {
        let mut pt = PieceTable::from("012");

        pt.insert(3, "345");
        pt.insert(6, "678");

        let change = pt.remove(3, 3);
        assert_eq!(pt.to_string(), "012678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![],
                old: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn removes_inside_span() {
        let mut pt = PieceTable::from("012345678");

        let change = pt.remove(3, 3);
        assert_eq!(pt.to_string(), "012678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Original,
                        position: 6,
                        length: 3,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 9,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn removes_with_both_positions_inside_span() {
        let mut pt = PieceTable::from("012");

        pt.insert(3, "345");
        pt.insert(6, "678");

        let change = pt.remove(2, 5);
        assert_eq!(pt.to_string(), "0178");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 2,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 4,
                        length: 2,
                        line_breaks: None
                    }
                ],
                old: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 3,
                        length: 3,
                        line_breaks: None
                    }
                ]
            }
        );
    }

    #[test]
    fn removes_with_start_position_inside_span() {
        let mut pt = PieceTable::from("012");

        pt.insert(3, "345");
        pt.insert(6, "678");

        let change = pt.remove(2, 4);
        assert_eq!(pt.to_string(), "01678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 2,
                    line_breaks: None
                }],
                old: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    }
                ]
            }
        );
    }

    #[test]
    fn removes_with_end_position_inside_span() {
        let mut pt = PieceTable::from("012");

        pt.insert(3, "345");
        pt.insert(6, "678");

        let change = pt.remove(3, 4);
        assert_eq!(pt.to_string(), "01278");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 4,
                    length: 2,
                    line_breaks: None
                }],
                old: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 3,
                        length: 3,
                        line_breaks: None
                    }
                ]
            }
        );
    }

    // # Replaces

    #[test]
    fn replaces_at_the_start() {
        let mut pt = PieceTable::from("6789345");

        let change = pt.replace(0, 4, "012");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Original,
                        position: 4,
                        length: 3,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 7,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn replaces_at_the_end() {
        let mut pt = PieceTable::from("0126789");

        let change = pt.replace(3, 4, "345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 7,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn replaces_at_position_beyond_end() {
        let mut pt = PieceTable::from("012");

        let change = pt.replace(6, 4, "345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![]
            }
        );
    }

    #[test]
    fn replaces_with_length_beyond_end() {
        let mut pt = PieceTable::from("012678");

        let change = pt.replace(3, 6, "345");
        assert_eq!(pt.to_string(), "012345");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Original,
                    position: 0,
                    length: 6,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn replaces_at_span_boundary() {
        let mut pt = PieceTable::from("012");

        pt.append("xxx");
        pt.append("678");

        let change = pt.replace(3, 3, "345");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![Span {
                    source: Source::Added,
                    position: 6,
                    length: 3,
                    line_breaks: None
                }],
                old: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 3,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn replaces_inside_span() {
        let mut pt = PieceTable::from("012");

        pt.append("3xxx8");

        let change = pt.replace(4, 3, "4567");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 1,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 5,
                        length: 4,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 4,
                        length: 1,
                        line_breaks: None
                    }
                ],
                old: vec![Span {
                    source: Source::Added,
                    position: 0,
                    length: 5,
                    line_breaks: None
                }]
            }
        );
    }

    #[test]
    fn replaces_with_both_positions_inside_span() {
        let mut pt = PieceTable::from("012");

        pt.append("3xx");
        pt.append("x8");

        let change = pt.replace(4, 3, "4567");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 1,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 5,
                        length: 4,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 4,
                        length: 1,
                        line_breaks: None
                    }
                ],
                old: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 3,
                        length: 2,
                        line_breaks: None
                    }
                ]
            }
        );
    }

    #[test]
    fn replaces_with_start_position_inside_span() {
        let mut pt = PieceTable::from("01x");

        pt.append("xxx");
        pt.append("678");

        let change = pt.replace(2, 4, "2345");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 0,
                new: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 2,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 6,
                        length: 4,
                        line_breaks: None
                    }
                ],
                old: vec![
                    Span {
                        source: Source::Original,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    }
                ]
            }
        );
    }

    #[test]
    fn replaces_with_end_position_inside_span() {
        let mut pt = PieceTable::from("012");

        pt.append("xxx");
        pt.append("x78");

        let change = pt.replace(3, 4, "3456");
        assert_eq!(pt.to_string(), "012345678");
        assert_eq!(
            change,
            Change {
                index: 1,
                new: vec![
                    Span {
                        source: Source::Added,
                        position: 6,
                        length: 4,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 4,
                        length: 2,
                        line_breaks: None
                    }
                ],
                old: vec![
                    Span {
                        source: Source::Added,
                        position: 0,
                        length: 3,
                        line_breaks: None
                    },
                    Span {
                        source: Source::Added,
                        position: 3,
                        length: 3,
                        line_breaks: None
                    }
                ]
            }
        );
    }

    // # Change

    #[test]
    fn applies_changes() {
        let mut pt = PieceTable::from("012");

        let change = pt.append("345");
        assert_eq!(pt.to_string(), "012345");

        pt.revert_change(&change);
        assert_eq!(pt.to_string(), "012");

        pt.apply_change(&change);
        assert_eq!(pt.to_string(), "012345");
    }

    // # Fetching content

    #[test]
    fn fetches_content_range() {
        let mut pt = PieceTable::from("012");

        pt.append("\r\n345");
        pt.append("\n678");

        let content = pt.range_as_string(0, 11);
        assert_eq!(content, "012\r\n345\n678");

        let content = pt.range_as_string(0, 1);
        assert_eq!(content, "0");

        let content = pt.range_as_string(4, 3);
        assert_eq!(content, "345");

        let content = pt.range_as_string(3, 3);
        assert_eq!(content, "\r\n34");

        let content = pt.range_as_string(5, 10);
        assert_eq!(content, "45\n678");
    }
}
