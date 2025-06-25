pub use iced::keyboard::{Key, Modifiers};
use std::collections::HashMap;

pub type Keymaps = HashMap<String, Keymap>;

#[derive(Debug, Clone, PartialEq)]
pub enum KeymapNode {
    Keymap(Keymap),
    Command(String),
}

// Pressing a key dispatches a message bassed on current keymap.
//
// To support key sequences, a keymap may contain an inner keymap.
// In such case, activating outer keymap swaps current keymap with the inner one.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Keymap {
    pub name: String,
    pub mapping: HashMap<(Key, Modifiers), KeymapNode>,
}

impl Keymap {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
