use druid::widget::{Container, Flex, Focus, FocusScope, SizedBox};
use druid::{Color, Data, Lens, Widget, WidgetExt, WidgetId};

use crate::ui::kit::TerminalTextboxDecorator;
use crate::ui::widgets::{AccessorDecorator, EditableText, Either};

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

pub fn build_ui() -> impl Widget<DebugState> {
    Either::new(
        |data, _| data.show_terminal,
        build_terminal(),
        SizedBox::empty(),
    )
}

fn build_terminal() -> impl Widget<DebugState> {
    FocusScope::new(
        Flex::column()
            .with_child(
                SizedBox::new(
                    Container::new(SizedBox::empty().expand())
                        .background(Color::rgba(0.0, 0.0, 0.0, 0.7)),
                )
                .height(150.0),
            )
            .with_child(
                SizedBox::new(
                    Container::new(build_command_input())
                        .background(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                )
                .width(f64::INFINITY),
            ),
    )
}

fn build_command_input() -> impl Widget<DebugState> {
    AccessorDecorator::new(
        TerminalTextboxDecorator::new(),
        Focus::new(
            EditableText::new()
                .with_placeholder("Enter the command")
                .lens(DebugState::terminal_command),
        )
        .with_id(TERMINAL_WIDGET_ID),
    )
    .padding(2.0)
}
