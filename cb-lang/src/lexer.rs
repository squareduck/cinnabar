enum ParserError {}

struct Span {
    start: usize,
    end: usize,
}

enum Token<'lex> {
    ListOpen,
    ListClose,
    Int(&'lex str),
    End,
}

struct Lexer<'lex> {
    input: &'lex str,
    position: usize,
    codemap_offset: usize,
}

impl<'lex> Lexer<'lex> {
    fn new(input: &'lex str, codemap_offset: usize) -> Self {
        Self {
            input,
            position: 0,
            codemap_offset,
        }
    }
}

impl<'lex> Iterator for Lexer<'lex> {
    type Item = Result<(Span, Token<'lex>), ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.char_indices();

        while let Some((index, ch)) = chars.next() {
            let (token, length) = match ch {
                '(' => (Token::ListOpen, 1),
                _ => (Token::End, 0),
            };
        }

        None
    }
}
