#[derive(Debug, Clone)]
pub enum Message {
    CreateWorkspace,
    DestroyWorkspace,
    ScrollDown,
    ScrollUp,
    GrowColumns,
    ShrinkColumns,
    GrowRows,
    ShrinkRows,
    ToggleModal,
}
