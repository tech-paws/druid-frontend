use druid::widget::{Align, Container, Flex, Focus, FocusScope, Label, SizedBox, Split};
use druid::{AppLauncher, Color, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

mod theme;
mod ui;

use crate::ui::widgets::AccessorDecorator;
use crate::ui::widgets::EditableText;
// use crate::ui::widgets::Focus;
// use crate::ui::widgets::FocusScope;
use crate::ui::widgets::Stack;
use crate::ui::widgets::TextBox;

use crate::ui::kit::ButtonDecorator;
use crate::ui::kit::FocusDecorator;
use crate::ui::kit::TerminalTextboxDecorator;
use crate::ui::kit::TextboxDecorator;

const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
    title: String,
}

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    let initial_state = HelloState {
        name: "".into(),
        title: "".into(),
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| theme::init(env))
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    let terminal_textbox = AccessorDecorator::new(
        TerminalTextboxDecorator::new(),
        Focus::new(
            EditableText::new()
                .with_placeholder("Enter the command")
                .fix_width(TEXT_BOX_WIDTH)
                .lens(HelloState::name),
        ),
    )
    .padding(2.0);

    let button = Focus::new(AccessorDecorator::new(
        FocusDecorator::new(),
        AccessorDecorator::new(
            ButtonDecorator::new(),
            Align::centered(Label::new("Content").padding((8.0, 2.0))).fix_width(100.0),
        )
        .fix_height(24.0)
        .on_click(|_, _, _| println!("Hello World!"))
        .padding(2.0),
    ))
    .with_auto_focus(true);

    let input = TextBox::new()
        .with_placeholder("Hint")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    let input2 = TextBox::new()
        .with_placeholder("")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::title);

    let layout = Stack::new()
        .with_child(FocusScope::new(
            Split::columns(
                Container::new(SizedBox::empty().expand())
                    .background(Color::rgb8(0xFF, 0xFF, 0xFF))
                    .rounded(4.0),
                Container::new(Align::centered(
                    Flex::column()
                        .with_child(Align::centered(button))
                        .with_child(SizedBox::empty().height(8.0))
                        .with_child(input)
                        .with_child(SizedBox::empty().height(8.0))
                        .with_child(input2),
                ))
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
        ))
        .with_child(FocusScope::new(
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
                        Container::new(terminal_textbox)
                            .background(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                    )
                    .width(f64::INFINITY),
                ),
        ));

    Align::centered(layout)
}
