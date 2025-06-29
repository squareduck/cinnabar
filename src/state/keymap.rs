pub use iced::keyboard::{Key, Modifiers};
use std::collections::HashMap;

use crate::state::{State, mode::Mode};

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

pub fn keymap_for_mode<'a>(state: &'a State, mode: &'a Mode) -> Option<&'a Keymap> {
    match mode {
        Mode::Workspace { .. } => state.keymaps.get("workspace-mode"),
        Mode::View { .. } => state.keymaps.get("view-mode"),
        _ => None,
    }
}

pub fn resolve_keybind(state: &State, (key, modifiers): (Key, Modifiers)) -> Option<String> {
    if let Some(keymap) = keymap_for_mode(state, &state.mode) {
        match keymap.mapping.get(&(key.clone(), modifiers)) {
            Some(KeymapNode::Command(handle)) => return Some(handle.clone()),
            _ => {}
        }
    }

    if let Some(keymap) = state.keymaps.get("global") {
        match keymap.mapping.get(&(key.clone(), modifiers)) {
            Some(KeymapNode::Command(handle)) => return Some(handle.clone()),
            _ => {
                eprintln!("Keybind not found for {:?} - {:?}", key, modifiers);
                eprintln!("Available global keybinds:");
                for ((key, modifiers), node) in &keymap.mapping {
                    eprintln!("  {:?} -> {:?}", (key, modifiers), node);
                }
                eprintln!("Available mode keybinds:");
                if let Some(mode_keymap) = keymap_for_mode(state, &state.mode) {
                    for ((key, modifiers), node) in &mode_keymap.mapping {
                        eprintln!("  {:?} -> {:?}", (key, modifiers), node);
                    }
                } else {
                    eprintln!("No keymap for current mode: {:?}", state.mode);
                }
            }
        }
    }

    None
}
