use iced::widget::text;

use crate::{
    message::Message,
    state::{Uid, tiling::Tiling},
};

pub type TiledItem<'a> = (Uid, String, iced::Element<'a, Message>);

enum FoldingDirection {
    Up,
    Down,
}

fn collapsed_rows<'a>(
    rows_count: usize,
    columns_count: usize,
    items_iter: &mut std::vec::IntoIter<TiledItem<'a>>,
    folding_direction: FoldingDirection,
) -> iced::Element<'a, Message> {
    use iced::Length;
    use iced::widget::{container, text};

    let mut column_element = iced::widget::Column::new();

    for row_index in 0..rows_count {
        let mut row_element = iced::widget::Row::new();

        for _ in 0..columns_count {
            if let Some((_id, title, _item)) = items_iter.next() {
                let is_folded_item = match folding_direction {
                    FoldingDirection::Up => row_index != rows_count - 1,
                    FoldingDirection::Down => row_index != 0,
                };

                row_element = if is_folded_item {
                    row_element.push(
                        iced::widget::container(iced::widget::Column::new())
                            .padding(5)
                            .height(Length::Shrink)
                            .width(Length::Fill)
                            .clip(true)
                            .style(container::bordered_box),
                    )
                } else {
                    row_element.push(
                        iced::widget::container(text(title))
                            .padding(10)
                            .max_height(50)
                            .width(Length::Fill)
                            .clip(true)
                            .style(container::bordered_box),
                    )
                };
            }
        }

        column_element = column_element.push(row_element);
    }

    column_element.into()
}

pub fn focused_box(theme: &iced::Theme) -> iced::widget::container::Style {
    use iced::widget::container::Style;

    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.weak.color.into()),
        border: iced::Border {
            width: 1.0,
            radius: 0.5.into(),
            color: iced::Color::from_rgb(0.0, 0.5, 1.0),
        },
        ..Style::default()
    }
}

pub fn expanded_rows<'a>(
    rows_count: usize,
    columns_count: usize,
    items_iter: &mut std::vec::IntoIter<TiledItem<'a>>,
    focused_id: Option<Uid>,
) -> iced::Element<'a, Message> {
    use iced::Length;
    use iced::widget::container;

    let mut column_element = iced::widget::Column::new();

    for _ in 0..rows_count {
        let mut row_element = iced::widget::Row::new();

        for _ in 0..columns_count {
            if let Some((id, _title, item)) = items_iter.next() {
                let mut item = iced::widget::container(item)
                    .padding(10)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .style(container::bordered_box);

                if focused_id == Some(id) {
                    item = item.style(focused_box);
                }

                row_element = row_element.push(item);
            }
        }

        column_element = column_element.push(row_element);
    }

    column_element.into()
}

pub fn tiled<'a>(
    tiling: &'a Tiling,
    items: Vec<(Uid, String, iced::Element<'a, Message>)>,
    modal_item: Option<iced::Element<'a, Message>>,
    focused_id: Option<Uid>,
) -> iced::Element<'a, Message> {
    use iced::Length;
    use iced::widget::{center, stack};

    let items_len = items.len();
    let mut items_iter = items.into_iter();

    let mut column = iced::widget::Column::new()
        .height(Length::Fill)
        .width(Length::Fill);

    if tiling.fullscreen {
        let item_container =
            if let Some((_id, _title, element)) = items_iter.find(|i| Some(i.0) == focused_id) {
                iced::widget::container(element)
                    .padding(10)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .style(focused_box)
            } else {
                iced::widget::container(text("No selection"))
                    .padding(10)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .style(focused_box)
            };

        column = column.push(item_container);
    } else {
        let max_row_count = items_len.div_ceil(tiling.max_columns);

        let expanded_rows_count = tiling.max_expanded_rows;
        let top_collapsed_rows_count = tiling.top_expanded_row_index;
        let bottom_collapsed_rows_count =
            max_row_count.saturating_sub(top_collapsed_rows_count + expanded_rows_count);

        if top_collapsed_rows_count > 0 {
            let sub_column = collapsed_rows(
                top_collapsed_rows_count,
                tiling.max_columns,
                &mut items_iter,
                FoldingDirection::Up,
            );

            column = column.push(sub_column);
        }

        if expanded_rows_count > 0 {
            let sub_column = expanded_rows(
                expanded_rows_count,
                tiling.max_columns,
                &mut items_iter,
                focused_id,
            );

            column = column.push(sub_column);
        }

        if bottom_collapsed_rows_count > 0 {
            let sub_column = collapsed_rows(
                bottom_collapsed_rows_count,
                tiling.max_columns,
                &mut items_iter,
                FoldingDirection::Down,
            );

            column = column.push(sub_column);
        }
    }

    if let Some(modal_item) = modal_item {
        stack![
            iced::widget::container(column)
                .width(Length::Fill)
                .height(Length::Fill),
            center(iced::widget::container(modal_item))
        ]
        .into()
    } else {
        iced::widget::container(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
