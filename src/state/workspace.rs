use anyhow::Result;
use iced::Task;
use thiserror::Error;

use crate::state::{
    State, Uid, command::CommandMap, create_uid, mode::ModeActions, tiling::Tiling,
};

#[derive(Error, Debug)]
pub enum WorkspaceError {
    #[error("Workspace with id {id} is not found")]
    NotFound { id: Uid },
    #[error("Default target for workspace action is not found")]
    NoTarget,
}

pub struct Workspace {
    pub id: Uid,
    pub activity_ids: Vec<Uid>,
    pub tiling: Tiling,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            id: create_uid(),
            activity_ids: Vec::new(),
            tiling: Tiling::default(),
        }
    }
}

pub trait WorkspaceActions {
    fn create_workspace(&mut self) -> Uid;
    fn delete_workspace(&mut self) -> Result<()>;
    fn next_workspace(&mut self) -> ();
    fn previous_workspace(&mut self) -> ();
}

impl WorkspaceActions for State {
    fn create_workspace(&mut self) -> Uid {
        let new_workspace_id = create_uid();
        self.screen.workspace_ids.push(new_workspace_id);
        self.workspaces
            .insert(new_workspace_id, Workspace::default());

        self.push_workspace_mode(Some(new_workspace_id));

        new_workspace_id
    }

    fn delete_workspace(&mut self) -> Result<()> {
        let mut previous_index = 0;

        if let Some(workspace_id) = self.current_workspace_id() {
            if let Some(index) = self
                .screen
                .workspace_ids
                .iter()
                .position(|&x| x == workspace_id)
            {
                self.screen.workspace_ids.remove(index);
                self.workspaces.remove(&workspace_id);

                previous_index = if index > 0 { index - 1 } else { 0 };
            } else {
                return Err(WorkspaceError::NoTarget.into());
            }
        }

        if self.screen.workspace_ids.is_empty() {
            self.update_workspace_mode(None);
        } else {
            self.update_workspace_mode(self.screen.workspace_ids.get(previous_index).copied());
        }

        Ok(())
    }

    fn next_workspace(&mut self) -> () {
        let next_workspace_id = if let Some(current_workspace_id) = self.current_workspace_id() {
            self.screen
                .workspace_ids
                .iter()
                .cycle()
                .skip_while(|workspace_id| **workspace_id != current_workspace_id)
                .skip(1)
                .next()
                .copied()
        } else {
            self.screen.workspace_ids.first().copied()
        };

        self.update_workspace_mode(next_workspace_id);
    }

    fn previous_workspace(&mut self) -> () {
        let next_workspace_id = if let Some(current_workspace_id) = self.current_workspace_id() {
            self.screen
                .workspace_ids
                .iter()
                .rev()
                .cycle()
                .skip_while(|workspace_id| **workspace_id != current_workspace_id)
                .skip(1)
                .next()
                .copied()
        } else {
            self.screen.workspace_ids.last().copied()
        };

        self.update_workspace_mode(next_workspace_id);
    }
}

pub fn workspace_commands() -> CommandMap {
    let mut commands = CommandMap::new();

    commands.insert_command(
        "workspace-create",
        "Create Workspace",
        "Create a new workspace",
        |state: &mut State| {
            state.create_workspace();
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "workspace-delete",
        "Delete Workspace",
        "Delete current workspace",
        |state: &mut State| {
            state.delete_workspace()?;
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "workspace-next",
        "Next Workspace",
        "Focus on next workspace",
        |state: &mut State| {
            state.next_workspace();
            Ok(Task::none())
        },
    );

    commands.insert_command(
        "workspace-previous",
        "Previous Workspace",
        "Focus on previous workspace",
        |state: &mut State| {
            state.previous_workspace();
            Ok(Task::none())
        },
    );
    commands
}
