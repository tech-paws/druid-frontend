use druid::{Data, Lens};

use crate::ui::scheme_editor::SchemeEditorState;
use crate::ui::debug::DebugState;

#[derive(Clone, Data, Lens)]
pub struct UiState {
    pub debug: DebugState,
    pub scheme_editor: SchemeEditorState,
}

impl UiState {
    pub fn new() -> Self {
        UiState {
            debug: DebugState::new(),
            scheme_editor: SchemeEditorState::new(),
        }
    }
}
