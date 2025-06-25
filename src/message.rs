use iced::keyboard::{Key, Modifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    KeyPress { key: Key, modifiers: Modifiers },
    Command(String),
    // OLD
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
