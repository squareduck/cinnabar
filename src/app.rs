use crate::state::State;
use crate::{message::Message, state::keymaps::KeymapNode};
use iced::{Subscription, Task, Theme, keyboard};

pub type App = State;

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        use crate::state::create_uid;

        let message = match message {
            Message::KeyPress { key, modifiers } => {
                if let Some(keymap) = self.keymap_for_mode(&self.mode) {
                    match keymap.mapping.get(&(key, modifiers)) {
                        Some(KeymapNode::Command(handle)) => Some(Message::Command(handle.clone())),
                        _ => None,
                    }
                } else if let Some(keymap) = self.keymaps.get("global") {
                    match keymap.mapping.get(&(key, modifiers)) {
                        Some(KeymapNode::Command(handle)) => Some(Message::Command(handle.clone())),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => Some(message),
        };

        let message = match message {
            Some(msg) => msg,
            None => return Task::none(),
        };

        match message {
            Message::Command(command) => {
                if let Some(command) = self.commands.get(&command) {
                    let _ = (command.command)(self);
                } else {
                    eprintln!("Command not found: {:?}", command);
                }

                Task::none()
            }
            Message::ScrollDown => {
                let max_row_count = self
                    .screen
                    .workspace_ids
                    .len()
                    .div_ceil(0.max(self.screen.tiling.max_columns));

                if max_row_count > self.screen.tiling.max_expanded_rows
                    && self.screen.tiling.top_expanded_row_index
                        < max_row_count - self.screen.tiling.max_expanded_rows
                {
                    self.screen.tiling.top_expanded_row_index += 1;
                }
                Task::none()
            }
            Message::ScrollUp => {
                if self.screen.tiling.top_expanded_row_index > 0 {
                    self.screen.tiling.top_expanded_row_index -= 1;
                }
                Task::none()
            }
            Message::GrowColumns => {
                self.screen.tiling.max_columns += 1;
                Task::none()
            }
            Message::ShrinkColumns => {
                if self.screen.tiling.max_columns > 1 {
                    self.screen.tiling.max_columns -= 1;
                }
                Task::none()
            }
            Message::GrowRows => {
                self.screen.tiling.max_expanded_rows += 1;
                Task::none()
            }
            Message::ShrinkRows => {
                if self.screen.tiling.max_expanded_rows > 1 {
                    self.screen.tiling.max_expanded_rows -= 1;
                }
                Task::none()
            }
            Message::ToggleModal => {
                if self.screen.transient_tool_id == None {
                    self.screen.transient_tool_id = Some(create_uid());
                } else {
                    self.screen.transient_tool_id = None;
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        use crate::elements::screen;

        screen(&self)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.screen.mode {
            // Focus::Tool { id, input_mode } if input_mode == &InputMode::Text => {
            //     // Input mode, subscribe to key presses
            //     keyboard::on_key_press(|key_code, modifiers| {
            //         // Handle key presses in input mode
            //         None
            //     })
            // }
            _ => {
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
                        _ => Some(Message::KeyPress {
                            key: key_code,
                            modifiers,
                        }),
                    }
                })
            }
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }
}
