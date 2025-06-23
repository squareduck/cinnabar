use iced::Task;

use crate::{
    commands::workspace,
    message::Message,
    state::{Focus, State},
};

pub fn create(state: &mut State) -> Task<Message> {
    match state.focus {
        Focus::Workspace {
            tiling_control: None,
            ..
        } => workspace::create(state),
        _ => Task::none(),
    }
}

pub fn cut(state: &mut State) -> Task<Message> {
    match state.focus {
        Focus::Workspace {
            id,
            tiling_control: None,
            ..
        } => workspace::cut(state, id),
        _ => Task::none(),
    }
}

pub fn next(state: &mut State) -> Task<Message> {
    match state.focus {
        Focus::Workspace {
            id,
            tiling_control: None,
            ..
        } => workspace::next(state, id),
        _ => Task::none(),
    }
}

pub fn previous(state: &mut State) -> Task<Message> {
    match state.focus {
        Focus::Workspace {
            id,
            tiling_control: None,
            ..
        } => workspace::previous(state, id),
        _ => Task::none(),
    }
}
