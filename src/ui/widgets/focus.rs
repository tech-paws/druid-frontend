// Copyright 2020 The druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A button widget.
use crate::theme;

use druid::widget::prelude::*;

use druid::{
    Data, HotKey, KbKey, Point, Rect, RenderContext, SysMods, Widget, WidgetPod,
};

pub struct Focus<T> {
    child: WidgetPod<T, Box<dyn Widget<T>>>,
    focus_node: FocusNode,
}

pub struct FocusNode {
    pub is_focused: bool,
}

impl FocusNode {
    pub fn new() -> Self {
        FocusNode { is_focused: false }
    }
}

impl<T: Data> Focus<T> {
    pub fn new(child: impl Widget<T> + 'static) -> Self {
        Focus {
            child: WidgetPod::new(child).boxed(),
            focus_node: FocusNode::new(),
        }
    }
}

impl<T: Data> Widget<T> for Focus<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.request_focus();
                ctx.request_paint();
            }
            Event::KeyDown(key_event) => {
                match key_event {
                    // Tab and shift+tab
                    k_e if HotKey::new(None, KbKey::Tab).matches(k_e) => {
                        ctx.focus_next();
                    }
                    k_e if HotKey::new(SysMods::Shift, KbKey::Tab).matches(k_e) => {
                        ctx.focus_prev();
                    }
                    _ => (),
                };

                ctx.request_paint();
            }
            _ => (),
        }

        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => ctx.register_for_focus(),
            LifeCycle::FocusChanged(value) => {
                self.child.set_focus(*value);
                ctx.request_paint();
            }
            _ => (),
        }

        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Focus");

        let border_width = 2.0;
        let child_bc = bc.shrink((2.0 * border_width, 2.0 * border_width));
        let size = self.child.layout(ctx, &child_bc, data, env);

        let origin = Point::new(border_width, border_width);
        let rect = Rect::from_origin_size(origin, size);

        self.child.set_layout_rect(ctx, data, env, rect);

        let my_size = Size::new(
            size.width + 2.0 * border_width,
            size.height + 2.0 * border_width,
        );

        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = ctx.size();

        let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
            .inset(2.0 / -2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let border_color = env.get(theme::FOCUS_BORDER_COLOR);

        self.child.paint(ctx, data, env);

        if self.focus_node.is_focused != ctx.is_focused() {
            self.focus_node.is_focused = ctx.is_focused();
        }

        if self.focus_node.is_focused {
            // if ctx.is_focused() {
            ctx.stroke(rounded_rect, &border_color, 2.0);
        }
    }
}
