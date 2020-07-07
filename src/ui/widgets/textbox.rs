// Copyright 2018 The Druid Authors.
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

//! A textbox widget.

use druid::{
    Application, BoxConstraints, Cursor, Env, Event, EventCtx, HotKey, KbKey, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Rect, Selector, TimerToken, UpdateCtx, Widget, WidgetExt, WidgetPod,
};

use crate::theme;

use druid::kurbo::{Affine, Line, Point, RoundedRect, Size, Vec2};

use crate::ui::kit::decorators::*;
use crate::ui::widgets::AccessorDecorator;
use crate::ui::widgets::EditableText;
use crate::ui::widgets::Focus;
use crate::ui::widgets::editable_text::TEXT_BOX_PLACEHOLDER;

/// A widget that allows user text input.
pub struct TextBox {
    placeholder: String,
    child: WidgetPod<String, Box<dyn Widget<String>>>,
}

impl TextBox {
    /// Create a new TextBox widget
    pub fn new() -> TextBox {
        let editable_text = EditableText::new();
        let decorator = Focus::new(AccessorDecorator::new(
            FocusDecorator::new(),
            AccessorDecorator::new(TextboxDecorator::new(), editable_text).padding(1.0),
        ));

        Self {
            child: WidgetPod::new(decorator).boxed(),
            placeholder: String::new(),
        }
    }

    /// Builder-style method to set the `TextBox`'s placeholder text.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl<'a> Widget<String> for TextBox {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, data: &String, env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &String, env: &Env) -> Size {
        let size = self.child.layout(ctx, &bc, data, env);
        let rect = Rect::from_origin_size(Point::ORIGIN, size);
        self.child.set_layout_rect(ctx, data, env, rect);

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        let mut new_env = env.clone();
        new_env.set(TEXT_BOX_PLACEHOLDER, &self.placeholder);
        self.child.paint(ctx, data, &new_env);
    }
}

impl Default for TextBox {
    fn default() -> Self {
        TextBox::new()
    }
}
