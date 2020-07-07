use druid::widget::prelude::*;
use druid::{Widget, WidgetPod, Data, Rect, Point, FocusChain};

pub struct FocusScope<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
    focus_chain: FocusChain,
}

impl<T: Data> FocusScope<T> {
    pub fn new(child: impl Widget<T> + 'static) -> Self {
        FocusScope {
            child: WidgetPod::new(child).boxed(),
            focus_chain: FocusChain::new(),
        }
    }
}

impl<T: Data> Widget<T> for FocusScope<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
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
        self.child.paint(ctx, data, env);
    }
}
