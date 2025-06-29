use crate::state::{State, Uid};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum ViewMode {
    #[default]
    None,
    Workspace {
        id: Option<Uid>,
    },
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum TransientStatus {
    #[default]
    None,
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Mode {
    #[default]
    None,
    View {
        mode: ViewMode,
    },
    Workspace {
        id: Option<Uid>,
    },
    Activity {
        id: Option<Uid>,
    },
    Pane {
        id: Option<Uid>,
    },
    Tool {
        id: Option<Uid>,
        transient: TransientStatus,
    },
}

pub trait ModeActions {
    fn push_workspace_mode(&mut self, id: Option<Uid>);
    fn update_workspace_mode(&mut self, id: Option<Uid>);
    fn current_workspace_id(&mut self) -> Option<Uid>;
    fn push_view_workspace_mode(&mut self, id: Option<Uid>);
    fn update_view_workspace_mode(&mut self, id: Option<Uid>);
    fn current_view_mode(&mut self) -> Option<ViewMode>;
    fn pop_mode(&mut self);
}

impl ModeActions for State {
    fn push_workspace_mode(&mut self, workspace_id: Option<Uid>) {
        self.mode_history.push(self.mode.clone());
        self.update_workspace_mode(workspace_id);
    }

    fn update_workspace_mode(&mut self, workspace_id: Option<Uid>) {
        self.mode = Mode::Workspace { id: workspace_id };
    }

    fn current_workspace_id(&mut self) -> Option<Uid> {
        if let Mode::Workspace { id } = self.mode {
            id
        } else {
            None
        }
    }

    fn push_view_workspace_mode(&mut self, workspace_id: Option<Uid>) {
        self.mode_history.push(self.mode.clone());
        self.update_view_workspace_mode(workspace_id);
    }

    fn update_view_workspace_mode(&mut self, workspace_id: Option<Uid>) {
        self.mode = Mode::View {
            mode: ViewMode::Workspace { id: workspace_id },
        };
    }

    fn current_view_mode(&mut self) -> Option<ViewMode> {
        if let Mode::View { mode } = self.mode {
            Some(mode)
        } else {
            None
        }
    }

    fn pop_mode(&mut self) {
        if let Some(previous_mode) = self.mode_history.pop() {
            self.mode = previous_mode;
        } else {
            self.mode = Mode::None;
        }
    }
}
