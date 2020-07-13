pub use druid::theme::*;

use druid::{Color, Env, Key};

pub const BUTTON_COLOR: Key<Color> = Key::new("paws.tech.button_color");
pub const BUTTON_HOVER_COLOR: Key<Color> = Key::new("paws.tech.button_hover_color");
pub const BUTTON_CLICK_COLOR: Key<Color> = Key::new("paws.tech.button_click_color");
pub const BUTTON_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_text_color");
pub const BUTTON_HOVER_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_hover_text_color");
pub const BUTTON_CLICK_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_click_text_color");

pub const TEXT_BOX_COLOR: Key<Color> = Key::new("paws.tech.textbox_color");
pub const TEXT_BOX_HOVER_COLOR: Key<Color> = Key::new("paws.tech.textbox_hover_color");
pub const TEXT_BOX_CLICK_COLOR: Key<Color> = Key::new("paws.tech.textbox_click_color");
pub const TEXT_BOX_TEXT_COLOR: Key<Color> = Key::new("paws.tech.textbox_text_color");
pub const TEXT_BOX_HOVER_TEXT_COLOR: Key<Color> = Key::new("paws.tech.textbox_hover_text_color");
pub const TEXT_BOX_CLICK_TEXT_COLOR: Key<Color> = Key::new("paws.tech.textbox_click_text_color");
pub const TEXT_BOX_BORDER_COLOR: Key<Color> = Key::new("paws.tech.text_box_border_color");
pub const TEXT_BOX_SELECTION_TEXT_COLOR: Key<Color> = Key::new("paws.tech.text_box_selection_text_color");

pub const TERMINAL_TEXT_BOX_TEXT_COLOR: Key<Color> =
    Key::new("paws.tech.terminal_text_box_text_color");
pub const TERMINAL_TEXT_BOX_CURSOR_COLOR: Key<Color> =
    Key::new("paws.tech.terminal_text_box_cursor_color");
pub const TERMINAL_TEXT_BOX_SELECTION_COLOR: Key<Color> =
    Key::new("paws.tech.terminal_text_box_selection_color");
pub const TERMINAL_TEXT_BOX_SELECTION_TEXT_COLOR: Key<Color> =
    Key::new("paws.tech.terminal_text_box_selection_text_color");

// pub const TEXT_BOX_TEXT_COLOR: Key<Color> = Key::new("paws.tech.button_click_text_color");
pub const FOCUS_BORDER_COLOR: Key<Color> = Key::new("paws.tech.focus_border_color");

pub fn init(env: &mut Env) {
    env.set(WINDOW_BACKGROUND_COLOR, Color::rgb8(0x43, 0x43, 0x43));
    env.set(BORDER_LIGHT, Color::rgba8(0x00, 0x00, 0x00, 0x00));
    env.set(LABEL_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(CURSOR_COLOR, Color::rgb8(0x3F, 0x8F, 0xFF));
    env.set(SELECTION_COLOR, Color::rgb8(0x3F, 0x8F, 0xFF));
    env.set(TEXT_BOX_SELECTION_TEXT_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));

    // env.set(BUTTON_LEAVE_COLOR, Color::rgba8(0xFF, 0xFF, 0xFF, 0xEE));
    env.set(BUTTON_COLOR, Color::rgba8(0xFF, 0xFF, 0xFF, 0xAA));
    env.set(BUTTON_HOVER_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(BUTTON_CLICK_COLOR, Color::rgb8(0x3F, 0x8F, 0xFF));
    env.set(BUTTON_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(BUTTON_HOVER_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(BUTTON_CLICK_TEXT_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));

    env.set(TEXT_BOX_COLOR, Color::rgba8(0xFF, 0xFF, 0xFF, 0xAA));
    env.set(TEXT_BOX_HOVER_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(TEXT_BOX_CLICK_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(TEXT_BOX_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(TEXT_BOX_HOVER_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(TEXT_BOX_CLICK_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    // env.set(TEXT_BOX_PLACEHOLDER, String::new());

    env.set(TEXT_SIZE_NORMAL, 12.0);
    // env.set(TEXT_BOX_TEXT_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(TEXT_BOX_BORDER_COLOR, Color::rgb8(0xB8, 0xB8, 0xB8));
    // env.set(TEXT_BOX_BORDER_COLOR, Color::rgb8(0x00, 0x00, 0x00));
    env.set(FOCUS_BORDER_COLOR, Color::rgb8(0x3F, 0x8F, 0xFF));
    env.set(TERMINAL_TEXT_BOX_TEXT_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(
        TERMINAL_TEXT_BOX_CURSOR_COLOR,
        Color::rgb8(0xFF, 0xFF, 0xFF),
    );
    env.set(TERMINAL_TEXT_BOX_SELECTION_COLOR, Color::rgb8(0xFF, 0xFF, 0xFF));
    env.set(TERMINAL_TEXT_BOX_SELECTION_TEXT_COLOR, Color::rgb8(0x00, 0x00, 0x00));
}
