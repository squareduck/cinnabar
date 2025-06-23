use iced::Task;

use crate::{
    message::{Command, Message},
    state::State,
};

mod navigation;
mod workspace;

pub fn dispatch(state: &mut State, command: Command) -> Task<Message> {
    match command {
        Command::Create => navigation::create(state),
        Command::Cut => navigation::cut(state),
        Command::Next => navigation::next(state),
        Command::Previous => navigation::previous(state),
        _ => Task::none(),
    }
}
