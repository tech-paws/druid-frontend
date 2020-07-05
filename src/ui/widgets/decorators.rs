use crate::theme;
use crate::ui::widgets::accessor_decorator::{AccessorData, AccessorDecorator};
use druid::{Color, Point, Rect};

use druid::widget::prelude::*;
use druid::widget::BackgroundBrush;
use druid::{LinearGradient, UnitPoint};
use druid::piet::{FixedLinearGradient, GradientStop};

pub struct ButtonDecorator;

pub struct TextboxDecorator;

impl ButtonDecorator {
    pub fn new() -> Self {
        ButtonDecorator
    }
}

impl TextboxDecorator {
    pub fn new() -> Self {
        TextboxDecorator
    }
}

struct ButtonColors {
    bg_color: Color,
    text_color: Color,
}

impl ButtonColors {
    fn new(data: &AccessorData, env: &Env) -> Self {
        if data.is_active {
            ButtonColors {
                bg_color: env.get(theme::BUTTON_CLICK_COLOR),
                text_color: env.get(theme::BUTTON_CLICK_TEXT_COLOR),
            }
        }
        else if data.is_hot {
            ButtonColors {
                bg_color: env.get(theme::BUTTON_HOVER_COLOR),
                text_color: env.get(theme::BUTTON_HOVER_TEXT_COLOR),
            }
        }
        else {
            ButtonColors {
                bg_color: env.get(theme::BUTTON_COLOR),
                text_color: env.get(theme::BUTTON_TEXT_COLOR),
            }
        }
    }
}

struct TextboxColors {
    bg_color: Color,
    text_color: Color,
}

impl TextboxColors {
    fn new(data: &AccessorData, env: &Env) -> Self {
        if data.is_active || data.has_focus {
            TextboxColors {
                bg_color: env.get(theme::TEXT_BOX_CLICK_COLOR),
                text_color: env.get(theme::TEXT_BOX_CLICK_TEXT_COLOR),
            }
        }
        else if data.is_hot {
            TextboxColors {
                bg_color: env.get(theme::TEXT_BOX_HOVER_COLOR),
                text_color: env.get(theme::TEXT_BOX_HOVER_TEXT_COLOR),
            }
        }
        else {
            TextboxColors {
                bg_color: env.get(theme::TEXT_BOX_COLOR),
                text_color: env.get(theme::TEXT_BOX_TEXT_COLOR),
            }
        }
    }
}

impl AccessorDecorator for ButtonDecorator {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AccessorData, env: &Env) {
        let size = ctx.size();

        let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let colors = ButtonColors::new(data, env);
        ctx.fill(rounded_rect, &colors.bg_color);
    }

    fn set_env(&mut self, _ctx: &mut PaintCtx, data: &AccessorData, env: &mut Env) {
        let colors = ButtonColors::new(data, env);
        env.set(theme::LABEL_COLOR, colors.text_color);
    }
}

impl AccessorDecorator for TextboxDecorator {
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AccessorData, env: &Env) {
        let size = ctx.size();

        let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        let colors = TextboxColors::new(data, env);
        let gradient = LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            vec![
                GradientStop {
                    pos: 0.0,
                    color: Color::grey(0.8),
                },
                GradientStop {
                    pos: 0.05,
                    color: Color::grey(0.9),
                },
                GradientStop {
                    pos: 0.2,
                    color: Color::grey(1.0),
                },
            ],
        );

        ctx.fill(rounded_rect, &gradient);
        ctx.stroke(rounded_rect, &env.get(theme::TEXT_BOX_BORDER_COLOR), 1.0);
    }

    fn set_env(&mut self, _ctx: &mut PaintCtx, data: &AccessorData, env: &mut Env) {
        let colors = TextboxColors::new(data, env);
        env.set(theme::LABEL_COLOR, colors.text_color);
    }
}
