use druid::{Data, Lens, WidgetId};

pub const TERMINAL_WIDGET_ID: WidgetId = WidgetId::reserved(1);

#[derive(Clone, Data, Lens)]
pub struct DebugState {
    pub terminal_command: String,
    pub show_terminal: bool,
}

impl DebugState {
    pub fn new() -> Self {
        DebugState {
            show_terminal: false,
            terminal_command: "".into(),
        }
    }
}
