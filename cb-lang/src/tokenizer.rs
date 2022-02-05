use unicode_segmentation::UnicodeSegmentation;

pub struct CodeSpan {
    token: Token,
    position: usize,
    length: usize,
}

pub enum Token {
    ListOpen,
    ListClose,
    Integer,
}

pub fn tokenize<T>(source: T) -> Vec<CodeSpan>
where
    T: Into<String>,
{
    let source = source.into();

    let chars = source
        .graphemes(true)
        .enumerate()
        .collect::<Vec<(usize, &str)>>();
    let mut spans = vec![];

    for (position, c) in chars {
        let span = match c {
            "(" => list_open(position),
            ")" => list_close(position),
            _ => panic!("unexpected character")
        };

        spans.push(span);
    }
    spans
}

fn list_open(position: usize) -> CodeSpan {
    CodeSpan {
        token: Token::ListOpen,
        position,
        length: 1,
    }
}

fn list_close(position: usize) -> CodeSpan {
    CodeSpan {
        token: Token::ListClose,
        position,
        length: 1,
    }
}