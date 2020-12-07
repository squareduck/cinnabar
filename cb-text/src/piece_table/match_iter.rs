use crate::piece_table::{PieceTable, SpanAddress};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
pub struct Match {
    position: usize,
    length: usize,
}

pub struct MatchIter<'pt> {
    piece_table: &'pt PieceTable,
    regex: Regex,
    position: usize,
    buffer: String,
}

const STEP: usize = 100;

impl<'pt> MatchIter<'pt> {
    pub fn new(piece_table: &'pt PieceTable, position: usize, regex: Regex) -> Self {
        MatchIter {
            piece_table,
            regex,
            position,
            buffer: piece_table.to_string(),
        }
    }
}

impl<'pt> Iterator for MatchIter<'pt> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        match self.regex.find(&self.buffer[self.position..]) {
            Some(m) => {
                let graphemes_before_match =
                    UnicodeSegmentation::graphemes(&self.buffer[..self.position + m.start()], true)
                        .count();
                let graphemes_inside_match = UnicodeSegmentation::graphemes(
                    &self.buffer[self.position + m.start()..self.position + m.end()],
                    true,
                )
                .count();
                self.position += m.end();

                Some(Match {
                    position: graphemes_before_match,
                    length: graphemes_inside_match,
                })
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn finds_graphemes() {
        let mut pt = PieceTable::from("0\r\n12\n345");

        let regex = Regex::new("\\w+").unwrap();
        let matches: Vec<Match> = pt.find_matches(0, regex).collect();
        assert_eq!(
            matches,
            vec![
                Match {
                    position: 0,
                    length: 1,
                },
                Match {
                    position: 2,
                    length: 2,
                },
                Match {
                    position: 5,
                    length: 3,
                }
            ]
        );
    }
}
