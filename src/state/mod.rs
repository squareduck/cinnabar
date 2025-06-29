pub mod command;
pub mod keymap;
pub mod mode;
pub mod screen;
pub mod tiling;
pub mod view;
pub mod workspace;

use anyhow::Error;

use self::mode::Mode;
use self::screen::Screen;
use self::workspace::Workspace;

use crate::state::command::{Command, CommandActions, CommandMap};
use crate::state::keymap::Keymaps;

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
    pub last_command: Option<Command>,
    pub errors: Vec<Error>,
}

impl State {
    pub fn push_error(&mut self, error: Error) {
        eprintln!("Error: {:?}", error);
        self.errors.push(error)
    }
}

impl Default for State {
    fn default() -> Self
    where
        Self: CommandActions,
    {
        let config = crate::config::Config::from_toml("./config.toml");

        Self {
            screen: Screen::default(),
            workspaces: HashMap::new(),
            mode: Mode::Workspace { id: None },
            mode_history: Vec::new(),
            commands: CommandMap::new(),
            keymaps: config.keymaps,
            last_command: None,
            errors: Vec::new(),
        }
    }
}
