use crate::state::Uid;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
pub enum ViewMode {
    #[default]
    View,
    Workspace {
        id: Option<Uid>,
    },
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
    },
}
