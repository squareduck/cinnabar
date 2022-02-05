use crate::history::HistoryTree;
use crate::piece_table::{Change, PieceTable};

pub struct Text {
    history: HistoryTree<Change>,
    piece_table: PieceTable,
}

impl Text {
    fn new(content: &str) -> Self {
        let mut piece_table = PieceTable::from(content);

        let change = piece_table.replace(0, 0, "");

        Text {
            history: HistoryTree::new(change),
            piece_table,
        }
    }
}
