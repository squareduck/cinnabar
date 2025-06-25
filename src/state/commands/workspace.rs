use iced::Task;

use crate::{
    message::Message,
    state::commands::{Command, CommandMap},
    state::{State, create_uid, mode::Mode},
};

pub fn commands() -> CommandMap {
    let mut commands = CommandMap::new();

    commands.insert(
        "workspace-create".to_string(),
        Command {
            handle: "workspace-create".to_string(),
            name: "Create Workspace".to_string(),
            description: "Create a new workspace".to_string(),
            command: create,
        },
    );

    commands.insert(
        "workspace-cut".to_string(),
        Command {
            handle: "workspace-cut".to_string(),
            name: "Cut Workspace".to_string(),
            description: "Cut the current workspace".to_string(),
            command: cut,
        },
    );

    commands.insert(
        "workspace-next".to_string(),
        Command {
            handle: "workspace-next".to_string(),
            name: "Next Workspace".to_string(),
            description: "Switch to the next workspace".to_string(),
            command: next,
        },
    );

    commands.insert(
        "workspace-previous".to_string(),
        Command {
            handle: "workspace-previous".to_string(),
            name: "Previous Workspace".to_string(),
            description: "Switch to the previous workspace".to_string(),
            command: previous,
        },
    );

    commands.insert(
        "workspace-mode".to_string(),
        Command {
            handle: "workspace-mode".to_string(),
            name: "Push Workspace Mode".to_string(),
            description: "Push the current workspace mode onto the mode history".to_string(),
            command: push_mode,
        },
    );

    commands
}

pub fn create(state: &mut State) -> Task<Message> {
    use crate::state::workspace::Workspace;

    let new_workspace_id = create_uid();
    state.screen.workspace_ids.push(new_workspace_id);
    state
        .workspaces
        .insert(new_workspace_id, Workspace::default());
    state.mode = Mode::Workspace {
        id: Some(new_workspace_id),
    };

    Task::none()
}

pub fn cut(state: &mut State) -> Task<Message> {
    let mut previous_index = 0;
    if let Mode::Workspace { id: Some(id), .. } = state.mode {
        if let Some(index) = state.screen.workspace_ids.iter().position(|&x| x == id) {
            state.screen.workspace_ids.remove(index);
            state.workspaces.remove(&id);

            previous_index = if index > 0 { index - 1 } else { 0 };
        }
    }

    if state.screen.workspace_ids.is_empty() {
        state.mode = Mode::Workspace { id: None };
    } else {
        state.mode = Mode::Workspace {
            id: state.screen.workspace_ids.get(previous_index).copied(),
        };
    }

    Task::none()
}

pub fn next(state: &mut State) -> Task<Message> {
    state.mode = if let Mode::Workspace { id: Some(id), .. } = state.mode {
        let next_workspace_id = state
            .screen
            .workspace_ids
            .iter()
            .cycle()
            .skip_while(|workspace_id| **workspace_id != id)
            .skip(1)
            .next();

        Mode::Workspace {
            id: next_workspace_id.copied(),
        }
    } else {
        Mode::Workspace {
            id: state.screen.workspace_ids.first().copied(),
        }
    };

    Task::none()
}

pub fn previous(state: &mut State) -> Task<Message> {
    state.mode = if let Mode::Workspace { id: Some(id), .. } = state.mode {
        let next_workspace_id = state
            .screen
            .workspace_ids
            .iter()
            .rev()
            .cycle()
            .skip_while(|workspace_id| **workspace_id != id)
            .skip(1)
            .next();

        Mode::Workspace {
            id: next_workspace_id.copied(),
        }
    } else {
        Mode::Workspace {
            id: state.screen.workspace_ids.last().copied(),
        }
    };

    Task::none()
}

pub fn push_mode(state: &mut State) -> Task<Message> {
    state.mode_history.push(state.mode.clone());
    state.mode = Mode::Workspace { id: None };

    Task::none()
}
