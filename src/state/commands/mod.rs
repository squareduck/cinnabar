use std::collections::HashMap;

use crate::{message::Message, state::State};

pub mod view;
pub mod workspace;

pub type CommandMap = HashMap<String, Command>;

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub handle: String,
    pub name: String,
    pub description: String,
    pub command: fn(&mut State) -> iced::Task<Message>,
}
