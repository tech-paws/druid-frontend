use druid::widget::{Align, Flex, Focus, FocusScope, Label, SizedBox, Split, TextBox};
use druid::{Color, Data, Lens, Widget, WidgetExt};

use crate::ui::renderer::Renderer;

use crate::ui::kit::{ButtonDecorator, FocusDecorator};
use crate::ui::widgets::AccessorDecorator;

#[derive(Clone, Data, Lens)]
pub struct SchemeEditorState {
    pub input1: String,
    pub input2: String,
}

impl SchemeEditorState {
    pub fn new() -> Self {
        SchemeEditorState {
            input1: String::new(),
            input2: String::new(),
        }
    }
}

pub fn build_ui() -> impl Widget<SchemeEditorState> {
    Renderer::new()
}

fn _demo_build_ui() -> impl Widget<SchemeEditorState> {
    let button = Focus::new(AccessorDecorator::new(
        FocusDecorator::new(),
        AccessorDecorator::new(
            ButtonDecorator::new(),
            Align::centered(Label::new("Content").padding((8.0, 7.0))).fix_width(100.0),
        )
        .fix_height(24.0)
        .on_click(|_, _, _| println!("Hello World!"))
        .padding(2.0),
    )).with_auto_focus(true);

    let input = TextBox::new()
        .with_placeholder("Hint")
        .fix_width(100.)
        .lens(SchemeEditorState::input1);

    let input2 = TextBox::new()
        .with_placeholder("")
        .fix_width(100.)
        .lens(SchemeEditorState::input2);

    FocusScope::new(
        Split::columns(
            // Container::new(SizedBox::empty().expand())
            //     .background(Color::rgb8(0xFF, 0xFF, 0xFF))
            //     .rounded(4.0),
            Renderer::new(),
            // Align::centered(
            Flex::column()
                .with_child(SizedBox::empty().height(400.0))
                .with_child(button)
                .with_child(SizedBox::empty().height(8.0))
                .with_child(input)
                .with_child(SizedBox::empty().height(8.0))
                .with_child(input2)
                // )
                .fix_height(f64::INFINITY)
                .background(Color::rgb8(0xD8, 0xD8, 0xD8))
                .rounded(4.0),
        )
        .split_point(0.7)
        .draggable(true)
        .solid_bar(true)
        .bar_size(3.0)
        .min_bar_area(3.0)
        .min_size(60.0),
    )
}
