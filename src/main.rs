use druid::widget::{Align, Container, Flex, Focus, FocusScope, Label, SizedBox, Split};
use druid::{
    commands, AppDelegate, AppLauncher, Color, Command, Data, DelegateCtx, Env, Event, HotKey,
    KbKey, Lens, LocalizedString, Target, Widget, WidgetExt, WidgetId, WindowDesc, WindowId,
};
use std::num::NonZeroU64;

mod theme;
mod ui;
mod work_area;

use crate::ui::widgets::AccessorDecorator;
use crate::ui::widgets::EditableText;
// use crate::ui::widgets::Focus;
// use crate::ui::widgets::FocusScope;
use crate::ui::widgets::Stack;
use crate::ui::widgets::TextBox;
use crate::ui::widgets::Either;

use crate::ui::kit::ButtonDecorator;
use crate::ui::kit::FocusDecorator;
use crate::ui::kit::TerminalTextboxDecorator;
use crate::ui::kit::TextboxDecorator;

use crate::work_area::WorkArea;
use tech_paws_core;

const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<SchemeEditorState> = LocalizedString::new("Hello World!");
const TERMINAL_WIDGET_ID: WidgetId = WidgetId::reserved(1);

#[derive(Clone, Data, Lens)]
struct SchemeEditorState {
    terminal_command: String,
    input1: String,
    input2: String,
    show_terminal: bool,
}

struct TechPawsAppDelegate {}

impl TechPawsAppDelegate {
    fn new() -> Self {
        TechPawsAppDelegate {}
    }
}

impl AppDelegate<SchemeEditorState> for TechPawsAppDelegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        data: &mut SchemeEditorState,
        _env: &Env,
    ) -> Option<Event> {
        match &event {
            Event::KeyDown(key_event) => {
                match key_event {
                    k_e if HotKey::new(None, KbKey::Character("`".into())).matches(k_e) => {
                        data.show_terminal = !data.show_terminal;
                        data.terminal_command = "".into();
                        ctx.submit_command(
                            Command::new(commands::REQUEST_FOCUS, TERMINAL_WIDGET_ID),
                            Target::Widget(TERMINAL_WIDGET_ID),
                        );
                        None
                    }
                    _ => Some(event),
                }
            }
            _ => Some(event),
        }
    }
}

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((1024.0, 700.0));

    let initial_state = SchemeEditorState {
        terminal_command: "".into(),
        input1: "".into(),
        input2: "".into(),
        show_terminal: false,
    };

    tech_paws_core::init_world();

    AppLauncher::with_window(main_window)
        .delegate(TechPawsAppDelegate::new())
        .configure_env(|env, _| theme::init(env))
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<SchemeEditorState> {
    let terminal_textbox = AccessorDecorator::new(
        TerminalTextboxDecorator::new(),
        Focus::new(
            EditableText::new()
                .with_placeholder("Enter the command")
                .fix_width(TEXT_BOX_WIDTH)
                .lens(SchemeEditorState::terminal_command),
        )
        .with_id(WidgetId::reserved(1)),
    )
    .padding(2.0);

    let button = Focus::new(AccessorDecorator::new(
        FocusDecorator::new(),
        AccessorDecorator::new(
            ButtonDecorator::new(),
            Align::centered(Label::new("Content").padding((8.0, 7.0))).fix_width(100.0),
        )
        .fix_height(24.0)
        .on_click(|_, _, _| println!("Hello World!"))
        .padding(2.0),
    ));

    let input = TextBox::new()
        .with_placeholder("Hint")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(SchemeEditorState::input1);

    let input2 = TextBox::new()
        .with_placeholder("")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(SchemeEditorState::input2);

    let terminal = FocusScope::new(
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
                    Container::new(terminal_textbox).background(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                )
                .width(f64::INFINITY),
            ),
    );

    let layout = Stack::new()
        .with_child(FocusScope::new(
            Split::columns(
                // Container::new(SizedBox::empty().expand())
                //     .background(Color::rgb8(0xFF, 0xFF, 0xFF))
                //     .rounded(4.0),
                WorkArea::new(),
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
        ))
        .with_child(Either::new(
            |data, _| data.show_terminal,
            terminal,
            SizedBox::empty(),
        ));

    Align::centered(layout)
}
