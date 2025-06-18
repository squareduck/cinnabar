use crate::{message::Message, state::TilingMode};
use iced::{
    Length,
    widget::{center, container, stack, text},
};

pub type TiledItem<'a> = (String, iced::Element<'a, Message>);

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
    let mut column_element = iced::widget::Column::new();

    for row_index in 0..rows_count {
        let mut row_element = iced::widget::Row::new();

        for _ in 0..columns_count {
            if let Some((title, _item)) = items_iter.next() {
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

pub fn expanded_rows<'a>(
    rows_count: usize,
    columns_count: usize,
    items_iter: &mut std::vec::IntoIter<TiledItem<'a>>,
) -> iced::Element<'a, Message> {
    let mut column_element = iced::widget::Column::new();

    for _ in 0..rows_count {
        let mut row_element = iced::widget::Row::new();

        for _ in 0..columns_count {
            if let Some((_title, item)) = items_iter.next() {
                row_element = row_element.push(
                    iced::widget::container(item)
                        .padding(10)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .style(container::bordered_box),
                );
            }
        }

        column_element = column_element.push(row_element);
    }

    column_element.into()
}

pub fn tiled<'a>(
    tiling_mode: &'a TilingMode,
    items: Vec<(String, iced::Element<'a, Message>)>,
    modal_item: Option<iced::Element<'a, Message>>,
) -> iced::Element<'a, Message> {
    let max_row_count = items.len().div_ceil(tiling_mode.max_columns);

    let expanded_rows_count = tiling_mode.max_expanded_rows;
    let top_collapsed_rows_count = tiling_mode.top_expanded_row_index;
    let bottom_collapsed_rows_count =
        if (top_collapsed_rows_count + expanded_rows_count) <= max_row_count {
            max_row_count - (top_collapsed_rows_count + expanded_rows_count)
        } else {
            0
        };

    let mut items_iter = items.into_iter();

    let mut column = iced::widget::Column::new()
        .height(Length::Fill)
        .width(Length::Fill);

    if top_collapsed_rows_count > 0 {
        let sub_column = collapsed_rows(
            top_collapsed_rows_count,
            tiling_mode.max_columns,
            &mut items_iter,
            FoldingDirection::Up,
        );

        column = column.push(sub_column);
    }

    if expanded_rows_count > 0 {
        let sub_column = expanded_rows(
            expanded_rows_count,
            tiling_mode.max_columns,
            &mut items_iter,
        );

        column = column.push(sub_column);
    }

    if bottom_collapsed_rows_count > 0 {
        let sub_column = collapsed_rows(
            bottom_collapsed_rows_count,
            tiling_mode.max_columns,
            &mut items_iter,
            FoldingDirection::Down,
        );

        column = column.push(sub_column);
    }

    if let Some(modal_item) = modal_item {
        stack![
            container(column).width(Length::Fill).height(Length::Fill),
            center(container(modal_item))
        ]
        .into()
    } else {
        container(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
