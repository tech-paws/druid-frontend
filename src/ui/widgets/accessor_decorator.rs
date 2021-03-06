use druid::widget::prelude::*;

use druid::{Data, Point, Rect, Widget, WidgetPod};

#[derive(Clone, Data)]
pub struct AccessorData {
    pub is_hot: bool,
    pub is_active: bool,
    pub has_focus: bool,
}

pub struct AccessorDecoratorWidget<T, A: AccessorDecorator> {
    pub(crate) child: WidgetPod<T, Box<dyn Widget<T>>>,
    decorator: A,
}

impl<T: Data, A: AccessorDecorator> AccessorDecoratorWidget<T, A> {
    pub fn new(decorator: A, child: impl Widget<T> + 'static) -> Self {
        AccessorDecoratorWidget {
            child: WidgetPod::new(child).boxed(),
            decorator,
        }
    }
}

pub trait AccessorDecorator {
    fn paint_background(&mut self, _: &mut PaintCtx, _: &AccessorData, _: &Env) {}

    fn paint_foreground(&mut self, _: &mut PaintCtx, _: &AccessorData, _: &Env) {}

    fn set_env(&mut self, _: &mut PaintCtx, _: &AccessorData, _: &mut Env) {}
}

impl<T: Data, A: AccessorDecorator> Widget<T> for AccessorDecoratorWidget<T, A> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            _ => (),
        }

        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.child.layout(ctx, &bc, data, env);
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        self.child.set_layout_rect(ctx, data, env, rect);

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let accessor_data = AccessorData {
            is_hot: ctx.is_hot(),
            is_active: ctx.is_active(),
            has_focus: ctx.focus_node().is_focused,
        };

        self.decorator.paint_background(ctx, &accessor_data, env);

        let mut new_env = env.clone();
        self.decorator.set_env(ctx, &accessor_data, &mut new_env);
        self.child.paint(ctx, data, &new_env);

        self.decorator.paint_foreground(ctx, &accessor_data, env);
    }
}
