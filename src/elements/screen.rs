use crate::{
    elements::tiled,
    message::Message,
    state::{Focus, Screen, State, Workspace},
};
use iced::{
    Element, Length,
    widget::{column, container, row, text},
};

pub fn screen(state: &State) -> Element<Message> {
    let screen = &state.screen;
    column!(
        tiled(
            &screen.tiling,
            screen
                .workspace_ids
                .iter()
                .map(|id| {
                    (
                        *id,
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
            match state.focus {
                Focus::Workspace { id, .. } => id,
                _ => None,
            },
        ),
        row!(text(format!("Focus: {:?}", state.focus))),
    )
    .into()
}
