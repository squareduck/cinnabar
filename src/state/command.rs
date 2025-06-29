use anyhow::Result;
use std::collections::HashMap;

use iced::Task;

use crate::{
    message::Message,
    state::{State, mode::ModeActions},
};

pub type Action = fn(&mut State) -> Result<Task<Message>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    handle: String,
    name: String,
    description: String,
    action: Action,
}

impl Command {
    pub fn run(&self, state: &mut State) -> Task<Message> {
        match (self.action)(state) {
            Ok(task) => task,
            Err(err) => {
                state.push_error(err);
                Task::none()
            }
        }
    }

    pub fn handle(&self) -> &str {
        &self.handle
    }
}

pub struct CommandMap {
    commands: HashMap<String, Command>,
}

impl CommandMap {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn get_command(&self, handle: &str) -> Option<Command> {
        self.commands.get(handle).cloned()
    }

    pub fn insert_command<S: Into<String>>(
        &mut self,
        handle: S,
        name: S,
        description: S,
        action: Action,
    ) {
        let command = Command {
            handle: handle.into(),
            name: name.into(),
            description: description.into(),
            action,
        };
        self.commands.insert(command.handle.clone(), command);
    }

    pub fn command_values(&self) -> impl Iterator<Item = &Command> {
        self.commands.values()
    }
}

pub trait CommandActions {
    fn resolve_command(&self, handle: &str) -> Option<Command>;
    fn insert_command(&mut self, command: Command);
    fn merge_commands(&mut self, commands: CommandMap);
}

impl CommandActions for State {
    fn resolve_command(&self, handle: &str) -> Option<Command> {
        self.commands.get_command(handle).clone()
    }

    fn insert_command(&mut self, command: Command) {
        self.commands.insert_command(
            command.handle,
            command.name,
            command.description,
            command.action,
        );
    }

    fn merge_commands(&mut self, commands: CommandMap) {
        for command in commands.command_values() {
            self.commands.insert_command(
                command.handle.clone(),
                command.name.clone(),
                command.description.clone(),
                command.action,
            );
        }
    }
}

pub fn global_commands() -> CommandMap {
    let mut commands = CommandMap::new();

    commands.insert_command(
        "workspace-mode",
        "Workspace Mode",
        "Push workspace mode",
        |state: &mut State| {
            let workspace_id = state.current_workspace_id();
            state.push_workspace_mode(workspace_id);
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "view-mode",
        "View Mode",
        "Push view mode",
        |state: &mut State| {
            let workspace_id = state.current_workspace_id();
            state.push_view_workspace_mode(workspace_id);
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "pop-mode",
        "Pop Mode",
        "Pop current mode from mode stack",
        |state: &mut State| {
            state.pop_mode();
            Ok(Task::none())
        },
    );

    commands
}
