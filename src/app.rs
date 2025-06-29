use crate::state::State;
use crate::state::keymap::resolve_keybind;
use crate::{message::Message, state::command::CommandActions};
use iced::{Subscription, Task, Theme, keyboard};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Command with handle {handle:?} not found")]
    CommandNotFound { handle: String },
}

pub type App = State;

impl App {
    pub fn new() -> (Self, Task<Message>)
    where
        Self: CommandActions,
    {
        let mut state = State::default();

        state.merge_commands(crate::state::command::global_commands());
        state.merge_commands(crate::state::workspace::workspace_commands());
        state.merge_commands(crate::state::view::view_commands());

        (state, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let message = match message {
            Message::KeyPress { key, modifiers } => {
                resolve_keybind(self, (key, modifiers)).map(Message::Command)
            }
            _ => Some(message),
        };

        let message = match message {
            Some(msg) => msg,
            None => return Task::none(),
        };

        match message {
            Message::Command(handle) => {
                if let Some(command) = self.resolve_command(&handle) {
                    self.last_command = Some(command.clone());
                    command.run(self)
                } else {
                    self.push_error(
                        AppError::CommandNotFound {
                            handle: handle.clone(),
                        }
                        .into(),
                    );

                    Task::none()
                }
            }
            Message::ToggleModal => {
                // if self.screen.transient_tool_id == None {
                //     self.screen.transient_tool_id = Some(create_uid());
                // } else {
                //     self.screen.transient_tool_id = None;
                // }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        use crate::elements::screen;

        screen(self)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            // Handle key presses in command mode
            match key_code {
                iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => {
                    Some(Message::ToggleModal)
                }
                _ => Some(Message::KeyPress {
                    key: key_code,
                    modifiers,
                }),
            }
        })
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }
}
