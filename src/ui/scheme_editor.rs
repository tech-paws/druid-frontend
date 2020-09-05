use druid::{Data, Lens, Widget};

use crate::ui::renderer::Renderer;

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
