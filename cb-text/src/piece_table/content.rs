use unicode_segmentation::UnicodeSegmentation;

use crate::piece_table::span::Span;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Source {
    Original,
    Added,
}

// Append only content buffer
pub struct Content {
    pub source: Source,
    pub buffer: String,
}

impl Content {
    pub fn new(source: Source) -> Self {
        Content {
            source,
            buffer: String::new(),
        }
    }

    pub fn append(&mut self, content: &str) -> Span {
        let line_breaks = Self::line_breaks_for_content(&content);

        let span = Span {
            source: self.source,
            position: UnicodeSegmentation::graphemes(self.buffer.as_str(), true).count(),
            length: UnicodeSegmentation::graphemes(content, true).count(),
            line_breaks,
        };

        let a: Vec<(usize, &str)> = UnicodeSegmentation::graphemes(content, true)
            .enumerate()
            .collect();

        self.buffer.push_str(content);

        span
    }

    pub fn content_for_span(&self, span: &Span, offset: usize, limit: usize) -> &str {
        use std::ops::Range;

        let offset = std::cmp::min(offset, span.length);

        let length = if offset + limit < span.length {
            limit
        } else {
            span.length - offset
        };

        let range = UnicodeSegmentation::grapheme_indices(self.buffer.as_str(), true)
            .skip(span.position + offset)
            .take(length)
            .fold(
                None,
                |acc: Option<Range<usize>>, (index, grapheme)| match acc {
                    None => Some(index..index + grapheme.len()),
                    Some(range) => Some(range.start..index + grapheme.len()),
                },
            )
            .unwrap_or(0..0);

        &self.buffer[range]
    }

    fn line_breaks_for_content(content: &str) -> Option<Vec<usize>> {
        let line_breaks: Vec<usize> = UnicodeSegmentation::graphemes(content, true)
            .enumerate()
            .filter(|(_position, grapheme)| grapheme == &"\n" || grapheme == &"\r\n")
            .map(|(position, _grapheme)| position)
            .collect();

        if line_breaks.len() > 0 {
            Some(line_breaks)
        } else {
            None
        }
    }
}
