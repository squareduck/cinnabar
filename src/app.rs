use iced::{Subscription, Task, Theme, keyboard};

use crate::{
    elements,
    message::Message,
    state::{Focus, InputMode, State, create_uid},
};

pub type App = State;

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CreateWorkspace => self.screen.workspace_ids.push(create_uid()),
            Message::DestroyWorkspace => {
                self.screen.workspace_ids.pop();
            }
            Message::ScrollDown => {
                let max_row_count = self
                    .screen
                    .workspace_ids
                    .len()
                    .div_ceil(0.max(self.screen.tiling_mode.max_columns));

                if max_row_count > self.screen.tiling_mode.max_expanded_rows
                    && self.screen.tiling_mode.top_expanded_row_index
                        < max_row_count - self.screen.tiling_mode.max_expanded_rows
                {
                    self.screen.tiling_mode.top_expanded_row_index += 1;
                }
            }
            Message::ScrollUp => {
                if self.screen.tiling_mode.top_expanded_row_index > 0 {
                    self.screen.tiling_mode.top_expanded_row_index -= 1;
                }
            }
            Message::GrowColumns => self.screen.tiling_mode.max_columns += 1,
            Message::ShrinkColumns => {
                if self.screen.tiling_mode.max_columns > 1 {
                    self.screen.tiling_mode.max_columns -= 1;
                }
            }
            Message::GrowRows => self.screen.tiling_mode.max_expanded_rows += 1,
            Message::ShrinkRows => {
                if self.screen.tiling_mode.max_expanded_rows > 1 {
                    self.screen.tiling_mode.max_expanded_rows -= 1;
                }
            }
            Message::ToggleModal => {
                if self.screen.transient_tool_id == None {
                    self.screen.transient_tool_id = Some(create_uid());
                } else {
                    self.screen.transient_tool_id = None;
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> iced::Element<Message> {
        elements::screen(&self.screen)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.screen.focus {
            Focus::Tool { id, input_mode } if input_mode == &InputMode::Text => {
                // Input mode, subscribe to key presses
                keyboard::on_key_press(|key_code, modifiers| {
                    // Handle key presses in input mode
                    None
                })
            }
            _ => {
                // Command mode or no context, subscribe to key presses
                keyboard::on_key_press(|key_code, modifiers| {
                    // Handle key presses in command mode
                    match key_code {
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                            Some(Message::ScrollDown)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                            Some(Message::ScrollUp)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                            Some(Message::ShrinkColumns)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                            Some(Message::GrowColumns)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Shift) => {
                            Some(Message::ShrinkRows)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Space) => {
                            Some(Message::GrowRows)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Enter) => {
                            Some(Message::CreateWorkspace)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                            Some(Message::DestroyWorkspace)
                        }
                        iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab) => {
                            Some(Message::ToggleModal)
                        }
                        _ => None,
                    }
                })
            }
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }
}
