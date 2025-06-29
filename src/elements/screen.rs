use iced::Element;

use crate::message::Message;
use crate::state::State;
use crate::state::mode::ViewMode;

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
                        format!("- {}", id),
                        container(text(format!("worspace {}", id)))
                            .padding(10)
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .style(container::bordered_box)
                            .into(),
                    )
                })
                .collect(),
            screen.transient_tool_id.map(|id| {
                container(text(format!("transient tool {}", id)))
                    .padding(10)
                    .style(container::bordered_box)
                    .into()
            }),
            match state.mode {
                Mode::Workspace { id, .. } => id,
                Mode::View {
                    mode: ViewMode::Workspace { id },
                } => id,
                _ => None,
            },
        ),
        row!(text(format!(
            " MODE: {:?} CMD: {}",
            match state.mode {
                Mode::None => "-".to_string(),
                Mode::Workspace { id } => format!("WSP {:?}", id),
                Mode::View {
                    mode: ViewMode::Workspace { .. },
                } => "WSP VIEW".to_string(),
                _ => "UNKNOWN".to_string(),
            },
            if let Some(command) = &state.last_command {
                command.handle()
            } else {
                "-"
            }
        ))),
    )
    .into()
}
