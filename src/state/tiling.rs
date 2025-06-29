#[derive(Debug)]
pub struct Tiling {
    pub max_expanded_rows: usize,
    pub max_columns: usize,
    pub top_expanded_row_index: usize,
    pub fullscreen: bool,
}

impl Default for Tiling {
    fn default() -> Self {
        Self {
            max_expanded_rows: 2,
            max_columns: 3,
            top_expanded_row_index: 0,
            fullscreen: false,
        }
    }
}
