use crate::{message::Message, state::Workspace};
use iced::{Element, widget::text};

pub fn workspace(workspace: &Workspace) -> Element<Message> {
    text("Workspace management is not implemented yet.").into()
}
