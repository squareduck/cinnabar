use crate::state::{Uid, mode::Mode, tiling::Tiling};

#[derive(Default)]
pub struct Screen {
    pub workspace_ids: Vec<Uid>,
    pub transient_tool_id: Option<Uid>,
    pub tiling: Tiling,
    pub mode: Mode,
}
