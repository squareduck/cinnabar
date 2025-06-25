use std::{collections::HashMap, path::PathBuf};

use iced::keyboard::{Key, Modifiers};
use toml::Value;

use crate::state::keymaps::{Keymap, KeymapNode, Keymaps};

pub struct Config {
    pub keymaps: Keymaps,
}

fn parse_keybind(input: Vec<&str>) -> (Key, Modifiers) {
    let mut key = Key::Unidentified;
    let mut modifiers = Modifiers::empty();

    for part in input {
        match part {
            "ctrl" => modifiers.insert(Modifiers::CTRL),
            "shift" => modifiers.insert(Modifiers::SHIFT),
            "cmd" => modifiers.insert(Modifiers::COMMAND),
            "alt" => modifiers.insert(Modifiers::ALT),
            "space" => {
                key = Key::Named(iced::keyboard::key::Named::Space);
            }
            "tab" => {
                key = Key::Named(iced::keyboard::key::Named::Tab);
            }
            "enter" => {
                key = Key::Named(iced::keyboard::key::Named::Enter);
            }
            "esc" => {
                key = Key::Named(iced::keyboard::key::Named::Escape);
            }
            _ => {
                key = Key::Character(part.into());
            }
        }
    }

    (key, modifiers)
}

impl Config {
    pub fn from_toml(path: impl Into<PathBuf>) -> Self {
        use toml::Table;
        // read file
        let content =
            std::fs::read_to_string(path.into()).expect("Failed to read keymap configuration file");

        // parse toml
        let table = content.parse::<Table>().unwrap();

        let mut keymaps = HashMap::new();

        if let Some(Value::Table(keymaps_table)) = table.get("keymaps") {
            for (keymap_name, keymap_table) in keymaps_table {
                if let Some(keymap) = keymap_table.as_table() {
                    let mut keymap_instance = Keymap::new(keymap_name.clone());

                    for (command_handle, keybind) in keymap {
                        if let Some(msg_str) = keybind.as_str() {
                            let keys = msg_str.split("-").collect();

                            keymap_instance.mapping.insert(
                                parse_keybind(keys),
                                KeymapNode::Command(command_handle.clone()),
                            );
                        }
                    }
                    keymaps.insert(keymap_name.clone(), keymap_instance);
                }
            }
        }

        Self { keymaps }
    }
}
