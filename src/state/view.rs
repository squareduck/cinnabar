use anyhow::Result;
use iced::Task;
use thiserror::Error;

use crate::state::{
    State,
    command::CommandMap,
    mode::{ModeActions, ViewMode},
};

#[derive(Error, Debug)]
pub enum ViewError {
    #[error("Default target for view action is not found")]
    NoTarget,
}

pub trait ViewActions {
    fn view_expand_rows(&mut self) -> Result<()>;
    fn view_shrink_rows(&mut self) -> Result<()>;
    fn view_expand_columns(&mut self) -> Result<()>;
    fn view_shrink_columns(&mut self) -> Result<()>;
    fn view_scroll_down(&mut self) -> Result<()>;
    fn view_scroll_up(&mut self) -> Result<()>;
}

impl ViewActions for State
where
    State: ModeActions,
{
    fn view_expand_rows(&mut self) -> Result<()> {
        match self.current_view_mode() {
            Some(ViewMode::Workspace { .. }) => {
                self.screen.tiling.max_expanded_rows += 1;
            }
            _ => return Err(ViewError::NoTarget.into()),
        }

        Ok(())
    }

    fn view_shrink_rows(&mut self) -> Result<()> {
        match self.current_view_mode() {
            Some(ViewMode::Workspace { .. }) => {
                if self.screen.tiling.max_expanded_rows > 1 {
                    self.screen.tiling.max_expanded_rows -= 1;
                }
            }
            _ => return Err(ViewError::NoTarget.into()),
        }

        Ok(())
    }

    fn view_expand_columns(&mut self) -> Result<()> {
        match self.current_view_mode() {
            Some(ViewMode::Workspace { .. }) => {
                self.screen.tiling.max_columns += 1;
            }
            _ => return Err(ViewError::NoTarget.into()),
        }

        Ok(())
    }

    fn view_shrink_columns(&mut self) -> Result<()> {
        match self.current_view_mode() {
            Some(ViewMode::Workspace { .. }) => {
                if self.screen.tiling.max_columns > 1 {
                    self.screen.tiling.max_columns -= 1;
                }
            }
            _ => return Err(ViewError::NoTarget.into()),
        }

        Ok(())
    }

    fn view_scroll_down(&mut self) -> Result<()> {
        match self.current_view_mode() {
            Some(ViewMode::Workspace { .. }) => {
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
            }
            _ => return Err(ViewError::NoTarget.into()),
        }

        Ok(())
    }

    fn view_scroll_up(self: &mut State) -> Result<()> {
        match self.current_view_mode() {
            Some(ViewMode::Workspace { .. }) => {
                if self.screen.tiling.top_expanded_row_index > 0 {
                    self.screen.tiling.top_expanded_row_index -= 1;
                }
            }
            _ => return Err(ViewError::NoTarget.into()),
        }

        Ok(())
    }
}

pub fn view_commands() -> CommandMap {
    let mut commands = CommandMap::new();

    commands.insert_command(
        "view-expand-rows",
        "Expand View Rows",
        "Expand visible rows in current view",
        |state: &mut State| {
            state.view_expand_rows()?;
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "view-shrink-rows",
        "Shrink View Rows",
        "Shrink visible rows in current view",
        |state: &mut State| {
            state.view_shrink_rows()?;
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "view-expand-columns",
        "Expand View Columns",
        "Expand visible columns in current view",
        |state: &mut State| {
            state.view_expand_columns()?;
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "view-shrink-columns",
        "Shrink View Columns",
        "Shrink visible columns in current view",
        |state: &mut State| {
            state.view_shrink_columns()?;
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "view-scroll-down",
        "Scroll Down View",
        "Scroll down expanded rows in current view",
        |state: &mut State| {
            state.view_scroll_down()?;
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "view-scroll-up",
        "Scroll Up View",
        "Scroll up expanded rows in current view",
        |state: &mut State| {
            state.view_scroll_up()?;
            Ok(Task::none())
        },
    );

    commands
}
