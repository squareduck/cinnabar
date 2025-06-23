use std::collections::HashMap;

use iced::keyboard::{Key, Modifiers};

use crate::message::{Command, Keymap, KeymapNode, Message};

pub type Uid = uuid::Uuid;

pub fn create_uid() -> Uid {
    Uid::now_v7()
}

#[derive(Debug)]
pub struct Tiling {
    pub max_expanded_rows: usize,
    pub max_columns: usize,
    pub top_expanded_row_index: usize,
}

impl Default for Tiling {
    fn default() -> Self {
        Self {
            max_expanded_rows: 2,
            max_columns: 3,
            top_expanded_row_index: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TilingControl {
    Rows,
    Columns,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Focus {
    #[default]
    None,
    Workspace {
        id: Option<Uid>,
        tiling_control: Option<TilingControl>,
    },
    Activity {
        id: Option<Uid>,
    },
    Pane {
        id: Option<Uid>,
    },
    Tool {
        id: Option<Uid>,
    },
}

#[derive(Default)]
pub struct Screen {
    pub workspace_ids: Vec<Uid>,
    pub transient_tool_id: Option<Uid>,
    pub tiling: Tiling,
    pub focus: Focus,
}

#[derive(Default)]
pub struct Workspace {
    pub id: Uid,
    pub activity_ids: Vec<Uid>,
    pub tiling: Tiling,
}

pub struct State {
    pub screen: Screen,
    pub workspaces: HashMap<Uid, Workspace>,
    pub focus: Focus,
    pub keymap: Keymap,
    pub last_command: Option<Command>,
}

impl Default for State {
    fn default() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(
            (Key::Character("c".into()), Modifiers::empty()),
            KeymapNode::Message(Message::Command(Command::Create)),
        );
        mapping.insert(
            (Key::Character("c".into()), Modifiers::SHIFT),
            KeymapNode::Message(Message::Command(Command::Cut)),
        );
        mapping.insert(
            (Key::Character("n".into()), Modifiers::empty()),
            KeymapNode::Message(Message::Command(Command::Next)),
        );
        mapping.insert(
            (Key::Character("p".into()), Modifiers::empty()),
            KeymapNode::Message(Message::Command(Command::Previous)),
        );

        Self {
            screen: Screen::default(),
            workspaces: HashMap::new(),
            focus: Focus::Workspace {
                id: None,
                tiling_control: None,
            },
            keymap: Keymap {
                name: "workspace".to_string(),
                mapping,
            },
            last_command: None,
        }
    }
}
