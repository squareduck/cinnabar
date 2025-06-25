use iced::Element;

use crate::message::Message;
use crate::state::State;

pub fn screen(state: &State) -> Element<Message> {
    use crate::elements::tiled::tiled;
    use crate::state::mode::Mode;
    use iced::Length;
    use iced::widget::{column, container, row, text};

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
            match state.mode {
                Mode::Workspace { id, .. } => id,
                _ => None,
            },
        ),
        row!(text(format!(" {:?}", state.mode))),
    )
    .into()
}
