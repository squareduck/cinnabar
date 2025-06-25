pub mod commands;
pub mod keymaps;
pub mod mode;
pub mod screen;
pub mod tiling;
pub mod workspace;

use self::mode::Mode;
use self::screen::Screen;
use self::workspace::Workspace;

use crate::state::commands::{Command, CommandMap};
use crate::state::keymaps::Keymaps;

use std::collections::HashMap;

pub type Uid = uuid::Uuid;

pub fn create_uid() -> Uid {
    Uid::now_v7()
}

pub struct State {
    pub screen: self::screen::Screen,
    pub workspaces: HashMap<Uid, Workspace>,
    pub mode: Mode,
    pub mode_history: Vec<Mode>,
    pub keymaps: Keymaps,
    pub commands: CommandMap,
}

impl State {
    pub fn insert_command(&mut self, command: Command) {
        self.commands.insert(command.handle.clone(), command);
    }

    pub fn merge_commands(&mut self, commands: CommandMap) {
        for command in commands.into_values() {
            self.insert_command(command);
        }
    }

    pub fn keymap_for_mode(&self, mode: &Mode) -> Option<&keymaps::Keymap> {
        match mode {
            Mode::Workspace { .. } => self.keymaps.get("workspace-mode"),
            Mode::View { .. } => self.keymaps.get("view-mode"),
            _ => None,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let config = crate::config::Config::from_toml("./config.toml");

        let mut state = Self {
            screen: Screen::default(),
            workspaces: HashMap::new(),
            mode: Mode::Workspace { id: None },
            mode_history: Vec::new(),
            commands: HashMap::new(),
            keymaps: config.keymaps,
        };

        state.merge_commands(crate::state::commands::workspace::commands());

        eprintln!("Available commands: {:?}", state.commands.values());

        state
    }
}
