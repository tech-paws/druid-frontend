use druid::kurbo::BezPath;
use druid::kurbo::Line;
use druid::piet::{FontBuilder, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{Affine, Color, Data, LocalizedString, Point, Rect, TimerToken, WindowDesc};
use std::marker::PhantomData;
use std::time::Duration;

use tech_paws_core as core;

pub struct WorkArea<T> {
    timer_id: TimerToken,
    data: PhantomData<T>,

    camera_position: core::commands::Vec2f,
    current_color: core::commands::Color,

    vec2f_data: Vec<core::commands::Vec2f>,
    vec2i_data: Vec<core::commands::Vec2i>,
    color_data: Vec<core::commands::Color>,
    str_data: Vec<String>,
}

impl<T: Data> WorkArea<T> {
    pub fn new() -> Self {
        WorkArea {
            timer_id: TimerToken::INVALID,
            data: PhantomData,
            camera_position: core::commands::Vec2f::new(0., 0.),
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
                    core::commands::RenderCommandType::SetColorUniform => {
                        self.current_color = self.color_data[0];
                        self.color_data.clear();
                    }
                    _ => (),
                }
            }
        }
    }

    fn draw_text(&mut self, ctx: &mut PaintCtx) {
        let color = Color::rgba(
            self.current_color.r,
            self.current_color.g,
            self.current_color.b,
            self.current_color.a,
        );

        let font = ctx
            .text()
            .new_font_by_name("Segoe UI", 12.0)
            .build()
            .unwrap();

        for str in self.str_data.iter().rev() {
            let pos = self
                .vec2f_data
                .pop()
                .map(|vec| Point::new(vec.x as f64, vec.y as f64 + 12.))
                .unwrap_or(Point::new(0., 12.));

            let layout = ctx
                .text()
                .new_text_layout(&font, str, std::f64::INFINITY)
                .build()
                .unwrap();

            ctx.draw_text(&layout, pos, &color);
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

        for chunk in self.vec2f_data.as_slice().chunks(2) {
            if chunk.len() < 2 {
                break;
            }

            let p1 = chunk[0];
            let p2 = chunk[1];

            let line = Line::new(
                Point::new(
                    (p1.x as f64 + self.camera_position.x as f64).floor() + 0.5,
                    (p1.y as f64 + self.camera_position.y as f64).floor() + 0.5,
                ),
                Point::new(
                    (p2.x as f64 + self.camera_position.x as f64).floor() + 0.5,
                    (p2.y as f64 + self.camera_position.y as f64).floor() + 0.5,
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

        for chunk in self.vec2f_data.as_slice().chunks(2) {
            if chunk.len() < 2 {
                break;
            }

            let pos = chunk[0];
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
        self.camera_position = self.vec2f_data[0];
        self.flush();
    }

    fn flush(&mut self) {
        self.current_color = core::commands::Color::rgb(0., 0., 0.);
        self.vec2f_data.clear();
        self.vec2i_data.clear();
        self.color_data.clear();
        self.str_data.clear();
    }
}

impl<T: Data> Widget<T> for WorkArea<T> {
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
                    tech_paws_core::frame_start();
                    tech_paws_core::step();

                    self.flush();
                    self.handle_exec_commands();

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
        let size = bc.max();

        core::push_set_view_port_size_request_command(core::commands::Vec2i::new(
            size.width as i32,
            size.height as i32,
        ));

        size
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, _: &T, _env: &Env) {
        // Let's draw a picture with Piet!

        // Clear the whole widget with the color of your choice
        // (ctx.size() returns the size of the layout rect we're painting in)
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size).to_rounded_rect(4.0);

        ctx.clip(rect);
        ctx.fill(rect, &Color::WHITE);

        self.handle_render_commands(ctx);

        tech_paws_core::frame_end();

        // // Note: ctx also has a `clear` method, but that clears the whole context,
        // // and we only want to clear this widget's area.

        // // Create an arbitrary bezier path
        // let mut path = BezPath::new();
        // path.move_to(Point::ORIGIN);
        // path.quad_to((80.0, 90.0), (size.width, size.height));
        // // Create a color
        // let stroke_color = Color::rgb8(0, 128, 0);
        // // Stroke the path with thickness 1.0
        // ctx.stroke(path, &stroke_color, 1.0);

        // // Rectangles: the path for practical people
        // let rect = Rect::from_origin_size((10., 10.), (100., 100.));
        // // Note the Color:rgba8 which includes an alpha channel (7F in this case)
        // let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
        // ctx.fill(rect, &fill_color);

        // // Text is easy, if you ignore all these unwraps. Just pick a font and a size.
        // let font = ctx
        //     .text()
        //     .new_font_by_name("Segoe UI", 24.0)
        //     .build()
        //     .unwrap();
        // // Here's where we actually use the UI state
        // let layout = ctx
        //     .text()
        //     .new_text_layout(&font, "Hello world", std::f64::INFINITY)
        //     .build()
        //     .unwrap();

        // // Let's rotate our text slightly. First we save our current (default) context:
        // ctx.with_save(|ctx| {
        //     // Now we can rotate the context (or set a clip path, for instance):
        //     ctx.transform(Affine::rotate(0.1));
        //     ctx.draw_text(&layout, (80.0, 40.0), &fill_color);
        // });
        // // When we exit with_save, the original context's rotation is restored

        // // Let's burn some CPU to make a (partially transparent) image buffer
        // let image_data = make_image_data(256, 256);
        // let image = ctx
        //     .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
        //     .unwrap();
        // // The image is automatically scaled to fit the rect you pass to draw_image
        // ctx.draw_image(
        //     &image,
        //     Rect::from_origin_size(Point::ORIGIN, size),
        //     InterpolationMode::Bilinear,
        // );
    }
}
