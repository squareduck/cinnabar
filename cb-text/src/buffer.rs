use crate::history::HistoryTree;
use crate::piece_table::{Change, PieceTable};

struct Cursor {
    position: usize,
    length: usize,
}

struct Buffer {
    file_path: Option<String>,
    cursors: Vec<Cursor>,
    history: HistoryTree<Change>,
    piece_table: PieceTable,
}

impl Buffer {
    fn new(content: &str) -> Self {
        let cursor = Cursor {
            position: 0,
            length: 0,
        };

        let mut piece_table = PieceTable::from(content);

        let change = piece_table.replace(0, 0, "");

        Buffer {
            file_path: None,
            cursors: vec![cursor],
            history: HistoryTree::new(change),
            piece_table,
        }
    }
}
