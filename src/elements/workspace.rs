use iced::Element;

use crate::{message::Message, state::workspace::Workspace};

pub fn workspace(_workspace: &Workspace) -> Element<Message> {
    use iced::widget::text;
    text("Workspace management is not implemented yet.").into()
}
