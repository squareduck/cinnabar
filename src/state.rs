type Uid = uuid::Uuid;

pub fn create_uid() -> Uid {
    Uid::now_v7()
}

#[derive(Debug)]
pub struct TilingMode {
    pub max_expanded_rows: usize,
    pub max_columns: usize,
    pub top_expanded_row_index: usize,
}

impl Default for TilingMode {
    fn default() -> Self {
        Self {
            max_expanded_rows: 2,
            max_columns: 3,
            top_expanded_row_index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Command,
    Text,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Focus {
    #[default]
    None,
    Workspace {
        id: Uid,
    },
    Activity {
        id: Uid,
    },
    Pane {
        id: Uid,
    },
    Tool {
        id: Uid,
        input_mode: InputMode,
    },
}

#[derive(Default)]
pub struct Screen {
    pub workspace_ids: Vec<Uid>,
    pub transient_tool_id: Option<Uid>,
    pub tiling_mode: TilingMode,
    pub focus: Focus,
}

#[derive(Default)]
pub struct Workspace {
    pub id: Uid,
    pub activity_ids: Vec<Uid>,
    pub tiling_mode: TilingMode,
}

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}
