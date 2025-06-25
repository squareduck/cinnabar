use iced::Task;

use crate::{
    message::Message,
    state::{
        State,
        mode::{Mode, ViewMode},
    },
};

pub fn push_mode(state: &mut State) -> Task<Message> {
    state.mode_history.push(state.mode.clone());
    match state.mode {
        Mode::Workspace { id } => {
            state.mode = Mode::View {
                mode: ViewMode::Workspace { id },
            };
        }
        _ => {}
    }

    Task::none()
}
