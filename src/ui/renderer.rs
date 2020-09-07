use druid::kurbo::Line;
use druid::piet::{FontFamily, Text, TextAttribute, TextLayout, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Color, Data, Point, Rect, TimerToken};
use std::marker::PhantomData;
use std::time::Duration;

use tech_paws_core as core;

pub struct Renderer<T> {
    timer_id: TimerToken,
    data: PhantomData<T>,

    camera_position: [core::commands::Vec2f; 2],
    current_color: core::commands::Color,

    current_camera_id: usize,
    int32_data: Vec<i32>,
    vec2f_data: Vec<core::commands::Vec2f>,
    vec2i_data: Vec<core::commands::Vec2i>,
    color_data: Vec<core::commands::Color>,
    str_data: Vec<String>,
}

impl<T: Data> Renderer<T> {
    pub fn new() -> Self {
        Renderer {
            timer_id: TimerToken::INVALID,
            data: PhantomData,
            current_camera_id: 0,
            camera_position: [core::commands::Vec2f::zero(), core::commands::Vec2f::zero()],
            int32_data: Vec::new(),
            vec2f_data: Vec::new(),
            vec2i_data: Vec::new(),
            color_data: Vec::new(),
            str_data: Vec::new(),
            current_color: core::commands::Color::rgb(0., 0., 0.),
        }
    }

    fn handle_exec_commands(&mut self) {
        let commands = core::c_get_exec_commands();

        for i in 0..commands.length {
            // TODO: doc
            unsafe {
                let command = commands.items.offset(i as isize).as_ref().unwrap();

                match command.command_type {
                    core::commands::ExecutionCommandType::PushVec2f => {
                        self.vec2f_data.push(command.data.vec2f);
                    }
                    core::commands::ExecutionCommandType::PushInt32 => {
                        self.int32_data.push(command.data.int32);
                    }
                    core::commands::ExecutionCommandType::UpdateCameraPosition => {
                        self.update_camera_position();
                    }
                }
            }
        }
    }

    fn handle_render_commands(&mut self, ctx: &mut PaintCtx) {
        let commands = core::c_get_render_commands();

        for i in 0..commands.length {
            // TODO: doc
            unsafe {
                let command = commands.items.offset(i as isize).as_ref().unwrap();

                match command.command_type {
                    core::commands::RenderCommandType::PushColor => {
                        self.color_data.push(command.data.color);
                    }
                    core::commands::RenderCommandType::PushVec2f => {
                        self.vec2f_data.push(command.data.vec2f);
                    }
                    core::commands::RenderCommandType::PushInt32 => {
                        self.int32_data.push(command.data.int32);
                    }
                    core::commands::RenderCommandType::PushString => {
                        self.str_data.push(command.data.string.data_to_string());
                    }
                    core::commands::RenderCommandType::DrawText => {
                        self.draw_text(ctx);
                    }
                    core::commands::RenderCommandType::DrawLines => {
                        self.draw_lines(ctx);
                    }
                    core::commands::RenderCommandType::DrawQuads => {
                        self.draw_quads(ctx);
                    }
                    core::commands::RenderCommandType::DrawPoints => {
                        self.draw_points(ctx);
                    }
                    core::commands::RenderCommandType::SetCamera => {
                        self.current_camera_id = self.int32_data[0] as usize;
                        self.int32_data.clear();
                    }
                    core::commands::RenderCommandType::SetColorUniform => {
                        self.current_color = self.color_data[0];
                        self.color_data.clear();
                    }
                    _ => (),
                }
            }
        }
    }

    fn handle_render_state(&mut self, ctx: &mut PaintCtx) {
        let commands = core::c_get_render_commands();

        for i in 0..commands.length {
            // TODO: doc
            unsafe {
                let command = commands.items.offset(i as isize).as_ref().unwrap();

                match command.command_type {
                    core::commands::RenderCommandType::PushColor => {
                        self.color_data.push(command.data.color);
                    }
                    core::commands::RenderCommandType::PushVec2f => {
                        self.vec2f_data.push(command.data.vec2f);
                    }
                    core::commands::RenderCommandType::PushInt32 => {
                        self.int32_data.push(command.data.int32);
                    }
                    core::commands::RenderCommandType::PushString => {
                        self.str_data.push(command.data.string.data_to_string());
                    }
                    core::commands::RenderCommandType::DrawText => {
                        self.render_state_text(ctx);
                    }
                    core::commands::RenderCommandType::SetCamera => {
                        self.current_camera_id = self.int32_data[0] as usize;
                        self.int32_data.clear();
                    }
                    _ => (),
                }
            }
        }
    }

    fn render_state_text(&mut self, ctx: &mut PaintCtx) {
        let font = FontFamily::SYSTEM_UI;

        for str in self.str_data.iter().rev() {
            let layout = ctx
                .text()
                .new_text_layout(&str)
                .font(font.clone(), 12.)
                .build()
                .unwrap();

            let bounds = layout.image_bounds();

            core::push_text_size(core::commands::Vec2f::new(
                bounds.size().width as f32,
                bounds.size().height as f32,
            ));
        }

        self.flush();
    }

    fn draw_text(&mut self, ctx: &mut PaintCtx) {
        let color = Color::rgba(
            self.current_color.r,
            self.current_color.g,
            self.current_color.b,
            self.current_color.a,
        );

        let font = FontFamily::SYSTEM_UI;
        let camera_position = self.camera_position[self.current_camera_id];

        for str in self.str_data.iter().rev() {
            let pos = self
                .vec2f_data
                .pop()
                .map(|vec| {
                    Point::new(
                        (camera_position.x + vec.x) as f64,
                        (camera_position.y + vec.y) as f64,
                    )
                })
                .unwrap_or_else(|| Point::new(camera_position.x as f64, camera_position.y as f64));

            let layout = ctx
                .text()
                .new_text_layout(&str)
                .font(font.clone(), 12.)
                .default_attribute(TextAttribute::ForegroundColor(color.clone()))
                .build()
                .unwrap();

            ctx.draw_text(&layout, pos);
        }

        self.flush();
    }

    fn draw_lines(&mut self, ctx: &mut PaintCtx) {
        if self.vec2f_data.len() < 2 {
            self.flush();
            return;
        }

        let color = Color::rgba(
            self.current_color.r,
            self.current_color.g,
            self.current_color.b,
            self.current_color.a,
        );

        let camera_position = self.camera_position[self.current_camera_id];

        for chunk in self.vec2f_data.as_slice().chunks(2) {
            if chunk.len() < 2 {
                break;
            }

            let p1 = chunk[0];
            let p2 = chunk[1];

            let line = Line::new(
                Point::new(
                    (p1.x as f64 + camera_position.x as f64).floor() + 0.5,
                    (p1.y as f64 + camera_position.y as f64).floor() + 0.5,
                ),
                Point::new(
                    (p2.x as f64 + camera_position.x as f64).floor() + 0.5,
                    (p2.y as f64 + camera_position.y as f64).floor() + 0.5,
                ),
            );
            ctx.stroke(line, &color, 1.);
        }

        self.flush();
    }

    fn draw_quads(&mut self, ctx: &mut PaintCtx) {
        let color = Color::rgba(
            self.current_color.r,
            self.current_color.g,
            self.current_color.b,
            self.current_color.a,
        );
        let camera_position = self.camera_position[self.current_camera_id];

        for chunk in self.vec2f_data.as_slice().chunks(2) {
            if chunk.len() < 2 {
                break;
            }

            let pos = chunk[0] + camera_position;
            let size = chunk[1];

            let rect = Rect::from_origin_size(
                (pos.x as f64, pos.y as f64),
                (size.x as f64, size.y as f64),
            );
            ctx.fill(rect, &color);
        }

        self.flush();
    }

    fn draw_points(&mut self, _ctx: &mut PaintCtx) {
        self.flush();
    }

    fn update_camera_position(&mut self) {
        let camera_id = self.int32_data[0] as usize;
        self.camera_position[camera_id] = self.vec2f_data[0];
        self.flush();
    }

    fn flush(&mut self) {
        self.current_color = core::commands::Color::rgb(0., 0., 0.);
        self.int32_data.clear();
        self.vec2f_data.clear();
        self.vec2i_data.clear();
        self.color_data.clear();
        self.str_data.clear();
    }
}

impl<T: Data> Widget<T> for Renderer<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(point) => {
                core::push_on_touch_start_request_command(core::commands::Vec2f::new(
                    point.pos.x as f32,
                    point.pos.y as f32,
                ));
            }
            Event::MouseUp(point) => {
                core::push_on_touch_end_request_command(core::commands::Vec2f::new(
                    point.pos.x as f32,
                    point.pos.y as f32,
                ));
            }
            Event::MouseMove(point) => {
                core::push_on_touch_move_request_command(core::commands::Vec2f::new(
                    point.pos.x as f32,
                    point.pos.y as f32,
                ));
            }
            Event::WindowConnected => {
                ctx.request_paint();
                let deadline = Duration::from_nanos(16_666_666);
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    ctx.request_paint();
                    let deadline = Duration::from_nanos(16_666_666);
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, _env: &Env) {
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size).to_rounded_rect(4.0);

        core::push_set_view_port_size_request_command(core::commands::Vec2i::new(
            size.width as i32,
            size.height as i32,
        ));

        tech_paws_core::frame_start();

        tech_paws_core::step();
        self.handle_exec_commands();
        tech_paws_core::flush();

        tech_paws_core::render_pass1();
        self.handle_render_state(ctx);
        self.flush();

        tech_paws_core::flush();
        tech_paws_core::render_pass2();

        self.flush();
        tech_paws_core::render_state_flush();

        ctx.clip(rect);
        ctx.fill(rect, &Color::WHITE);

        self.handle_render_commands(ctx);

        tech_paws_core::frame_end();
    }
}
