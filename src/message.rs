use std::collections::HashMap;

use iced::keyboard::{Key, Modifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum KeymapNode {
    Keymap(Keymap),
    Message(Message),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Create,
    Cut,
    Next,
    Previous,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    KeyPress { key: Key, modifiers: Modifiers },
    Command(Command),
    SwitchKeymap { keymap: Keymap },
    // OLD
    CreateWorkspace,
    DestroyWorkspace,
    ScrollDown,
    ScrollUp,
    GrowColumns,
    ShrinkColumns,
    GrowRows,
    ShrinkRows,
    ToggleModal,
}
