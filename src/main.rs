use druid::widget::{Align, Container, Flex, Label, SizedBox, Split};
use druid::{
    AppLauncher, BoxConstraints, Color, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
    WindowDesc,
};

mod theme;
mod ui;

// use crate::ui::widgets::decorators::ButtonDecorator;
use crate::ui::widgets::AccessorDecorator;
use crate::ui::widgets::ButtonDecorator;
use crate::ui::widgets::TextboxDecorator;
use crate::ui::widgets::Focus;
use crate::ui::widgets::TextBox;

const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

// use crate::{
//     BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
//     UpdateCtx, Widget, WidgetPod,
// };

/// A container that lays out its children along the z-axis, first child at bottom, last child on top.
#[derive(Default)]
pub struct Stack<T> {
    children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
}

impl<T: Data> Stack<T> {
    /// Create a new stack layout.
    ///
    /// The child widgets are laid out on top of one another, from bottom to top.
    pub fn new() -> Self {
        Stack {
            children: Vec::new(),
        }
    }

    /// Builder-style variant of `add_child`.
    ///
    /// Convenient for assembling a group of widgets in a single expression.
    pub fn with_child(mut self, child: impl Widget<T> + 'static) -> Self {
        self.add_child(child);
        self
    }

    /// Add a child widget.
    ///
    /// See also `with_child`.
    pub fn add_child(&mut self, child: impl Widget<T> + 'static) {
        self.children.push(WidgetPod::new(child).boxed());
    }
}

impl<T: Data> Widget<T> for Stack<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            // Event::MouseDown(e) | Event::MouseUp(e) | Event::MouseMove(e) => {
            //     if let Some(active_child) = self
            //         .children
            //         .iter_mut()
            //         .rev()
            //         // .find(|child| child.layout_rect().contains(e.pos))
            //     {
            //         active_child.event(ctx, event, data, env);
            //     }
            // }
            _ => {
                for child in &mut self.children.iter_mut().rev() {
                    child.event(ctx, event, data, env);
                    if ctx.is_handled() {
                        break;
                    }
                }
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        for child in &mut self.children {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        for child in &mut self.children {
            child.update(ctx, data, env);
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Stack");
        let loosened_bc = bc.loosen();
        let mut max_width = 0.0f64;
        let mut max_height = 0.0f64;
        for child in &mut self.children {
            let child_size: Size = child.layout(ctx, &loosened_bc, data, env);
            max_width = max_width.max(child_size.width);
            max_height = max_height.max(child_size.height);
            // Stash size.
            let rect = child_size.to_rect();
            child.set_layout_rect(ctx, data, env, rect);
        }
        Size::new(max_width, max_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        for child in &mut self.children {
            child.paint(ctx, data, env);
        }
    }
}

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    let initial_state = HelloState {
        name: "World".into(),
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            theme::init(env);

            // pub const BUTTON_DARK: Key<Color> = Key::new("button_dark");
            // pub const BUTTON_LIGHT: Key<Color> = Key::new("button_light");
            // pub const BUTTON_BORDER_WIDTH: Key<f64> = Key::new("button_border_width");
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    // let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));

    let textbox = Focus::new(
        TextBox::new()
            .with_placeholder("Enter the command")
            .fix_width(TEXT_BOX_WIDTH)
            .lens(HelloState::name),
    )
    .padding(2.0);

    let textbox2 = Focus::new(
        TextBox::new()
            .with_placeholder("Who are we greeting?")
            .fix_width(TEXT_BOX_WIDTH)
            .lens(HelloState::name),
    )
    .padding(2.0);

    // let layout = Flex::column().with_flex_child(
    //     // Flex::row()
    //     //     .with_flex_child(
    //     //         Label::new("top left")
    //     //             .center()
    //     //             .padding(10.0),
    //     //         1.0,
    //     //     )
    //     //     .with_flex_child(
    //     //         Label::new("top right")
    //     //             .center()
    //     //             // .background(DARK_GREY)
    //     //             .padding(10.0),
    //     //         1.0,
    //     //     ),
    //     Split::columns(
    //         Container::new(Align::centered(Label::new("Content")))
    //             .background(Color::rgb8(0xFF, 0xFF, 0xFF))
    //             .rounded(4.0),
    //         Container::new(Align::centered(Label::new("Content")))
    //             .background(Color::rgb8(0xD8, 0xD8, 0xD8))
    //             .rounded(4.0),
    //     )
    //     .split_point(0.7)
    //     .draggable(true)
    //     .solid_bar(true)
    //     .bar_size(3.0)
    //     .min_bar_area(3.0)
    //     .min_size(60.0),
    //     1.0,
    // );

    let button = Focus::new(
        AccessorDecorator::new(
            ButtonDecorator::new(),
            Align::centered(
                Label::new("Content")
                    .padding((8.0, 2.0))
            )
            .fix_width(100.0),
        )
        .fix_height(24.0)
        .on_click(|_, _, _| println!("Hello World!")),
    );

    let input = AccessorDecorator::new(
        TextboxDecorator::new(),
        Focus::new(
            TextBox::new()
                .with_placeholder("Enter the command")
                .fix_width(TEXT_BOX_WIDTH)
                .lens(HelloState::name),
        ),
    )
    .padding(2.0);

    let layout = Stack::new()
        .with_child(
            Split::columns(
                Container::new(Align::centered(Label::new("Content")))
                    .background(Color::rgb8(0xFF, 0xFF, 0xFF))
                    .rounded(4.0),
                Container::new(
                    Align::centered(
                        Flex::column()
                            .with_child(Align::centered(button))
                            .with_child(SizedBox::empty().height(8.0))
                            .with_child(input)
                    ),
                )
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
        .with_child(
            Flex::column()
                .with_child(
                    SizedBox::new(
                        Container::new(Align::centered(Label::new("Content")))
                            .background(Color::rgba(0.0, 0.0, 0.0, 0.7)),
                    )
                    .height(150.0),
                )
                .with_child(
                    SizedBox::new(
                        Container::new(textbox).background(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                    )
                    // .height(26.0)
                    .width(f64::INFINITY),
                )
                .with_child(
                    SizedBox::new(
                        Container::new(textbox2).background(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                    )
                    // .height(26.0)
                    .width(f64::INFINITY),
                ),
        );

    // .with_child(label)
    // .with_spacer(VERTICAL_WIDGET_SPACING)
    // .with_child(textbox)
    // .with_spacer(VERTICAL_WIDGET_SPACING)
    // .with_child(textbox2);

    Align::centered(layout)
}
