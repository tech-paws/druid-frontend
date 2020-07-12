use crate::theme;
use crate::ui::widgets::accessor_decorator::{AccessorData, AccessorDecorator};
use druid::{Color, Point, Rect};

use druid::piet::GradientStop;
use druid::widget::prelude::*;
use druid::{LinearGradient, UnitPoint};

pub struct ButtonDecorator;

pub struct TextboxDecorator;

pub struct TerminalTextboxDecorator;

pub struct FocusDecorator;

impl ButtonDecorator {
    pub fn new() -> Self {
        ButtonDecorator
    }
}

impl FocusDecorator {
    pub fn new() -> Self {
        FocusDecorator
    }
}

impl TextboxDecorator {
    pub fn new() -> Self {
        TextboxDecorator
    }
}

impl TerminalTextboxDecorator {
    pub fn new() -> Self {
        TerminalTextboxDecorator
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
    fn paint_background(&mut self, ctx: &mut PaintCtx, data: &AccessorData, env: &Env) {
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
    fn paint_background(&mut self, ctx: &mut PaintCtx, data: &AccessorData, env: &Env) {
        let size = ctx.size();

        let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
            .inset(0.5)
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

impl AccessorDecorator for TerminalTextboxDecorator {
    fn set_env(&mut self, _ctx: &mut PaintCtx, _data: &AccessorData, env: &mut Env) {
        env.set(
            theme::LABEL_COLOR,
            env.get(theme::TERMINAL_TEXT_BOX_TEXT_COLOR),
        );
        env.set(
            theme::CURSOR_COLOR,
            env.get(theme::TERMINAL_TEXT_BOX_TEXT_COLOR),
        );
    }
}

impl AccessorDecorator for FocusDecorator {
    fn paint_foreground(&mut self, ctx: &mut PaintCtx, data: &AccessorData, env: &Env) {
        if data.has_focus {
            let size = ctx.size();

            let rounded_rect = Rect::from_origin_size(Point::ORIGIN, size)
                .inset(-2.0)
                .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

            let border_color = env.get(theme::FOCUS_BORDER_COLOR);

            ctx.stroke(rounded_rect, &border_color, 2.0);
        }
    }
}
