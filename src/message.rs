use iced::keyboard::{Key, Modifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    KeyPress { key: Key, modifiers: Modifiers },
    Command(String),
    // TODO: Refactor out into commands
    ToggleModal,
}
