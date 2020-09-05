use druid::{Data, Lens, Widget};

use crate::ui::renderer::Renderer;
use crate::ui::ui_state::UiState;

#[derive(Clone, Data, Lens)]
pub struct SchemeEditorState {}

impl SchemeEditorState {
    pub fn new() -> Self {
        SchemeEditorState {}
    }
}

pub fn build_ui() -> impl Widget<SchemeEditorState> {
    Renderer::new()
}
