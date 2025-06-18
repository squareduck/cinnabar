use crate::{
    elements::tiled,
    message::Message,
    state::{Screen, Workspace},
};
use iced::{
    Element, Length,
    widget::{container, text},
};

pub fn screen(screen: &Screen) -> Element<Message> {
    tiled(
        &screen.tiling_mode,
        screen
            .workspace_ids
            .iter()
            .map(|id| {
                (
                    format!("- {}", id.to_string()),
                    container(text(format!("worspace {}", id.to_string())))
                        .padding(10)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .style(container::bordered_box)
                        .into(),
                )
            })
            .collect(),
        screen.transient_tool_id.map(|id| {
            container(text(format!("transient tool {}", id.to_string())))
                .padding(10)
                .style(container::bordered_box)
                .into()
        }),
    )
}
