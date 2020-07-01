pub use druid::theme::*;

use druid::{Color, Env, Key};

pub const BUTTON_LEAVE_COLOR: Key<Color> = Key::new("paws.tech.button_leave_color");
pub const BUTTON_ENTER_COLOR: Key<Color> = Key::new("paws.tech.button_enter_color");
pub const BUTTON_CLICK_COLOR: Key<Color> = Key::new("paws.tech.button_click_color");
pub const BUTTON_LEAVE_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_leave_text_color");
pub const BUTTON_ENTER_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_enter_text_color");
pub const BUTTON_CLICK_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_click_text_color");

pub fn init(env: &mut Env) {
    env.set(WINDOW_BACKGROUND_COLOR, Color::rgb8(0x43, 0x43, 0x43));
    env.set(BORDER_LIGHT, Color::rgba8(0x00, 0x00, 0x00, 0x00));
    env.set(LABEL_COLOR, Color::rgb8(0x00, 0x00, 0x00));

    env.set(BUTTON_LEAVE_COLOR, Color::rgba8(0xFF, 0xFF, 0xFF, 0xEE));
    env.set(BUTTON_ENTER_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(BUTTON_CLICK_COLOR, Color::rgb8(0x3F, 0x8F, 0xFF));
    env.set(BUTTON_LEAVE_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(BUTTON_ENTER_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(BUTTON_CLICK_TEXT_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));

    env.set(TEXT_SIZE_NORMAL, 13.0);
}
