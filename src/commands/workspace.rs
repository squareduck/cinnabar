use iced::Task;

use crate::{
    message::Message,
    state::{Focus, State, Uid, create_uid},
};

pub fn create(state: &mut State) -> Task<Message> {
    let new_workspace_id = create_uid();
    state.screen.workspace_ids.push(new_workspace_id);
    state
        .workspaces
        .insert(new_workspace_id, crate::state::Workspace::default());
    state.focus = Focus::Workspace {
        id: Some(new_workspace_id),
        tiling_control: None,
    };

    Task::none()
}

pub fn cut(state: &mut State, id: Option<Uid>) -> Task<Message> {
    if let Some(id) = id {
        if let Some(index) = state.screen.workspace_ids.iter().position(|&x| x == id) {
            state.screen.workspace_ids.remove(index);
            state.workspaces.remove(&id);
        }
    }

    if state.screen.workspace_ids.is_empty() {
        state.focus = Focus::Workspace {
            id: None,
            tiling_control: None,
        };
    } else {
        state.focus = Focus::Workspace {
            id: state.screen.workspace_ids.first().copied(),
            tiling_control: None,
        };
    }

    Task::none()
}

pub fn next(state: &mut State, id: Option<Uid>) -> Task<Message> {
    let next_workspace = if let Some(id) = id {
        state
            .screen
            .workspace_ids
            .iter()
            .cycle()
            .skip_while(|workspace_id| **workspace_id != id)
            .skip(1)
            .next()
    } else {
        state.screen.workspace_ids.first()
    };

    state.focus = Focus::Workspace {
        id: next_workspace.copied(),
        tiling_control: None,
    };

    Task::none()
}

pub fn previous(state: &mut State, id: Option<Uid>) -> Task<Message> {
    let previous_workspace = if let Some(id) = id {
        state
            .screen
            .workspace_ids
            .iter()
            .rev()
            .cycle()
            .skip_while(|workspace_id| **workspace_id != id)
            .skip(1)
            .next()
    } else {
        state.screen.workspace_ids.last()
    };

    state.focus = Focus::Workspace {
        id: previous_workspace.copied(),
        tiling_control: None,
    };

    Task::none()
}
