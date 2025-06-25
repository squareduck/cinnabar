use crate::state::{Uid, create_uid, tiling::Tiling};

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
