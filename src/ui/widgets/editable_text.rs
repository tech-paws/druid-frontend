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

use std::time::Duration;

use druid::{
    Application, BoxConstraints, Cursor, Env, Event, EventCtx, HotKey, KbKey, Key, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, Selector, TimerToken, UpdateCtx, Widget,
};

use druid::commands;
use druid::TextLayout;

use crate::theme;
use druid::kurbo::{Affine, Insets, Line, Point, RoundedRect, Size, Vec2};
use druid::piet::{
    FontFamily, PietText, PietTextLayout, RenderContext, Text, TextAttribute, TextLayoutBuilder,
};

const TEXT_INSETS: Insets = Insets::new(4.0, 2.0, 0.0, 2.0);
const CURSOR_BLINK_DURATION: Duration = Duration::from_millis(500);

use druid::text::{
    movement, offset_for_delete_backwards, BasicTextInput, EditAction, MouseAction, Movement,
    Selection, TextInput,
};

use druid::text::EditableText as EditableTextExt;

const BORDER_WIDTH: f64 = 2.;
const PADDING_TOP: f64 = 6.;
const PADDING_LEFT: f64 = 4.;

// we send ourselves this when we want to reset blink, which must be done in event.
const RESET_BLINK: Selector = Selector::new("druid-builtin.reset-textbox-blink");
const CURSOR_BLINK_DRUATION: Duration = Duration::from_millis(500);
pub const TEXT_BOX_PLACEHOLDER: Key<&str> = Key::new("textbox-placeholder");

/// A widget that allows user text input.
#[derive(Debug, Clone)]
pub struct EditableText {
    text: TextLayout,
    placeholder: String,
    width: f64,
    hscroll_offset: f64,
    selection: Selection,
    cursor_timer: TimerToken,
    cursor_on: bool,
}

impl EditableText {
    /// Perform an `EditAction`. The payload *must* be an `EditAction`.
    pub const PERFORM_EDIT: Selector<EditAction> =
        Selector::new("druid-builtin.textbox.perform-edit");

    /// Create a new EditableText widget
    pub fn new() -> EditableText {
        Self {
            text: TextLayout::new(""),
            width: 0.0,
            hscroll_offset: 0.,
            selection: Selection::caret(0),
            cursor_timer: TimerToken::INVALID,
            cursor_on: false,
            placeholder: String::new(),
        }
    }

    /// Builder-style method to set the `EditableText`'s placeholder text.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Calculate the PietTextLayout from the given text, font, and font size
    // fn get_layout(
    //     &self,
    //     piet_text: &mut PietText,
    //     text: &str,
    //     env: &Env,
    //     use_placeholder_color: bool,
    //     use_selection_color: bool,
    // ) -> PietTextLayout {
    //     // let font_name = env.get(theme::REGULAR_FONT_NAME);
    //     let font_size = env.get(theme::TEXT_SIZE_NORMAL);
    //     // TODO: caching of both the format and the layout
    //     // let font = piet_text
    //     //     .new_font_by_name(font_name, font_size)
    //     //     .build()
    //     //     .unwrap();
    //     let font = piet_text
    //         .font_family("Hack")
    //         .unwrap_or(FontFamily::SYSTEM_UI);

    //     // let selection_text_color = env.get(theme::TEXT_BOX_SELECTION_TEXT_COLOR);
    //     // let text_color = env.get(theme::LABEL_COLOR);
    //     // let placeholder_color = env.get(theme::PLACEHOLDER_COLOR);
    //     let text_color = if use_selection_color {
    //         env.get(theme::TEXT_BOX_SELECTION_TEXT_COLOR)
    //     } else if use_placeholder_color {
    //         env.get(theme::PLACEHOLDER_COLOR)
    //     } else {
    //         env.get(theme::LABEL_COLOR)
    //     };

    //     // piet_text.
    //     //     .new_text_layout(&font, &text.to_string(), std::f64::INFINITY)
    //     //     .build()
    //     //     .unwrap()

    //     piet_text
    //         .new_text_layout(&text.to_string())
    //         .font(font, font_size)
    //         .default_attribute(TextAttribute::ForegroundColor(text_color))
    //         .build()
    //         .unwrap()

    //     // .range_attribute(
    //     //     self.selection.range(),
    //     //     TextAttribute::ForegroundColor(selection_text_color),
    //     // )
    // }

    /// Insert text at the cursor position.
    /// Replaces selected text if there's a selection.
    fn insert(&mut self, src: &mut String, new: &str) {
        // EditableText's edit method will panic if selection is greater than
        // src length, hence we try to constrain it.
        //
        // This is especially needed when data was modified externally.
        // TODO: perhaps this belongs in update?
        let selection = self.selection.constrain_to(src);

        src.edit(selection.range(), new);
        self.selection = Selection::caret(selection.min() + new.len());
    }

    /// Set the selection to be a caret at the given offset, if that's a valid
    /// codepoint boundary.
    fn caret_to(&mut self, text: &mut String, to: usize) {
        match text.cursor(to) {
            Some(_) => self.selection = Selection::caret(to),
            None => log::error!("You can't move the cursor there."),
        }
    }

    /// Return the active edge of the current selection or cursor.
    // TODO: is this the right name?
    fn cursor(&self) -> usize {
        self.selection.end
    }

    fn do_edit_action(&mut self, edit_action: EditAction, text: &mut String) {
        match edit_action {
            EditAction::Insert(chars) | EditAction::Paste(chars) => self.insert(text, &chars),
            EditAction::Backspace => self.delete_backward(text),
            EditAction::Delete => self.delete_forward(text),
            EditAction::JumpDelete(movement) => {
                self.move_selection(movement, text, true);
                self.delete_forward(text)
            }
            EditAction::JumpBackspace(movement) => {
                self.move_selection(movement, text, true);
                self.delete_backward(text)
            }
            EditAction::Move(movement) => self.move_selection(movement, text, false),
            EditAction::ModifySelection(movement) => self.move_selection(movement, text, true),
            EditAction::SelectAll => self.selection.all(text),
            // TODO: https://github.com/linebender/druid/pull/1092
            // EditAction::SelectNone => self.selection.none(),
            EditAction::Click(action) => {
                if action.mods.shift() {
                    self.selection.end = action.column;
                }
                else {
                    self.caret_to(text, action.column);
                }
            }
            EditAction::Drag(action) => self.selection.end = action.column,
        }
    }

    /// Edit a selection using a `Movement`.
    fn move_selection(&mut self, mvmnt: Movement, text: &mut String, modify: bool) {
        // This movement function should ensure all movements are legit.
        // If they aren't, that's a problem with the movement function.
        self.selection = movement(mvmnt, self.selection, text, modify);
    }

    /// Delete to previous grapheme if in caret mode.
    /// Otherwise just delete everything inside the selection.
    fn delete_backward(&mut self, text: &mut String) {
        if self.selection.is_caret() {
            let cursor = self.cursor();
            let new_cursor = offset_for_delete_backwards(&self.selection, text);
            text.edit(new_cursor..cursor, "");
            self.caret_to(text, new_cursor);
        }
        else {
            text.edit(self.selection.range(), "");
            self.caret_to(text, self.selection.min());
        }
    }

    fn delete_forward(&mut self, text: &mut String) {
        if self.selection.is_caret() {
            // Never touch the characters before the cursor.
            if text.next_grapheme_offset(self.cursor()).is_some() {
                self.move_selection(Movement::Right, text, false);
                self.delete_backward(text);
            }
        }
        else {
            self.delete_backward(text);
        }
    }

    /// For a given point, returns the corresponding offset (in bytes) of
    /// the grapheme cluster closest to that point.
    fn offset_for_point(&self, point: Point) -> usize {
        // Translating from screenspace to Piet's text layout representation.
        // We need to account for hscroll_offset state and TextBox's padding.
        let translated_point = Point::new(point.x + self.hscroll_offset - TEXT_INSETS.x0, point.y);
        self.text.text_position_for_point(translated_point)
    }

    /// Given an offset (in bytes) of a valid grapheme cluster, return
    /// the corresponding x coordinate of that grapheme on the screen.
    fn x_pos_for_offset(&self, offset: usize) -> f64 {
        self.text.point_for_text_position(offset).x
    }

    /// Calculate a stateful scroll offset
    fn update_hscroll(&mut self) {
        let cursor_x = self.x_pos_for_offset(self.cursor());
        let overall_text_width = self.text.size().width;

        // when advancing the cursor, we want some additional padding
        let padding = TEXT_INSETS.x0 * 2.;
        if overall_text_width < self.width {
            // There's no offset if text is smaller than text box
            //
            // [***I*  ]
            // ^
            self.hscroll_offset = 0.;
        }
        else if cursor_x > self.width + self.hscroll_offset - padding {
            // If cursor goes past right side, bump the offset
            //       ->
            // **[****I]****
            //   ^
            self.hscroll_offset = cursor_x - self.width + padding;
        }
        else if cursor_x < self.hscroll_offset {
            // If cursor goes past left side, match the offset
            //    <-
            // **[I****]****
            //   ^
            self.hscroll_offset = cursor_x
        }
    }

    fn reset_cursor_blink(&mut self, ctx: &mut EventCtx) {
        self.cursor_on = true;
        self.cursor_timer = ctx.request_timer(CURSOR_BLINK_DURATION);
    }
}

impl Widget<String> for EditableText {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        // Guard against external changes in data?
        self.selection = self.selection.constrain_to(data);

        // let mut text_layout = self.get_layout(&mut ctx.text(), &data, env, false, false);
        let mut edit_action = None;

        match event {
            Event::MouseDown(mouse) => {
                // ctx.request_focus();
                ctx.set_active(true);

                if !mouse.focus {
                    let cursor_offset = self.offset_for_point(mouse.pos);
                    edit_action = Some(EditAction::Click(MouseAction {
                        row: 0,
                        column: cursor_offset,
                        mods: mouse.mods,
                    }));
                }

                ctx.request_paint();
            }
            Event::MouseMove(mouse) => {
                ctx.set_cursor(&Cursor::IBeam);
                if ctx.is_active() {
                    let cursor_offset = self.offset_for_point(mouse.pos);
                    edit_action = Some(EditAction::Drag(MouseAction {
                        row: 0,
                        column: cursor_offset,
                        mods: mouse.mods,
                    }));
                    ctx.request_paint();
                }
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    ctx.request_paint();
                }
            }
            Event::Timer(id) => {
                if *id == self.cursor_timer {
                    self.cursor_on = !self.cursor_on;
                    ctx.request_paint();
                    self.cursor_timer = ctx.request_timer(CURSOR_BLINK_DRUATION);
                }
            }
            Event::Command(ref cmd)
                if ctx.is_focused()
                    && (cmd.is(druid::commands::COPY) || cmd.is(druid::commands::CUT)) =>
            {
                if let Some(text) = data.slice(self.selection.range()) {
                    Application::global().clipboard().put_string(text);
                }
                if !self.selection.is_caret() && cmd.is(druid::commands::CUT) {
                    edit_action = Some(EditAction::Delete);
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(commands::FOCUS_NODE_FOCUS_CHANGED) => {
                self.reset_cursor_blink(ctx);
                let is_focused = *cmd.get_unchecked(commands::FOCUS_NODE_FOCUS_CHANGED);

                if is_focused {
                    self.do_edit_action(EditAction::SelectAll, data);
                }
                else {
                    self.do_edit_action(EditAction::Move(Movement::StartOfDocument), data);
                }
            }
            Event::Command(cmd) if cmd.is(RESET_BLINK) => self.reset_cursor_blink(ctx),
            Event::Command(cmd) if cmd.is(EditableText::PERFORM_EDIT) => {
                let edit = cmd.get_unchecked(EditableText::PERFORM_EDIT);
                self.do_edit_action(edit.to_owned(), data);
            }
            Event::Paste(ref item) => {
                if let Some(string) = item.get_string() {
                    edit_action = Some(EditAction::Paste(string));
                    ctx.request_paint();
                }
            }
            Event::KeyDown(key_event) => {
                let event_handled = match key_event {
                    k_e if HotKey::new(None, KbKey::Enter).matches(k_e) => {
                        // 'enter' should do something, maybe?
                        // but for now we are suppressing it, because we don't want
                        // newlines.
                        true
                    }
                    _ => false,
                };

                if !event_handled {
                    edit_action = BasicTextInput::new().handle_event(key_event);
                }

                ctx.request_paint();
            }
            _ => (),
        }

        if let Some(edit_action) = edit_action {
            let is_select_all = matches!(edit_action, EditAction::SelectAll);

            self.do_edit_action(edit_action, data);
            self.reset_cursor_blink(ctx);
            if data.is_empty() {
                self.text.set_text(self.placeholder.as_str());
            }
            else {
                self.text.set_text(data.as_str());
            }
            self.text.rebuild_if_needed(ctx.text(), env);

            if !is_select_all {
                self.update_hscroll();
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                if data.is_empty() {
                    self.text.set_text(self.placeholder.as_str());
                    self.text.set_text_color(theme::PLACEHOLDER_COLOR);
                }
                else {
                    self.text.set_text(data.as_str());
                }
                self.text.rebuild_if_needed(ctx.text(), env);
            }
            // an open question: should we be able to schedule timers here?
            LifeCycle::FocusChanged(true) => {
                // TODO: Send from Focus widget
                ctx.submit_command(RESET_BLINK.to(ctx.widget_id()));
            }
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, _data: &String, env: &Env) {
        self.text.rebuild_if_needed(ctx.text(), env);
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &String,
        env: &Env,
    ) -> Size {
        let width = env.get(theme::WIDE_WIDGET_WIDTH);
        let height = env.get(theme::BORDERED_WIDGET_HEIGHT);

        let size = bc.constrain((width, height));
        self.width = size.width;
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        // let placeholder = String::from_str(env.try_get(TEXT_BOX_PLACEHOLDER).unwrap_or(""));

        // Guard against changes in data following `event`
        // let content = if data.is_empty() {
        //     if self.placeholder.is_empty() {
        //         &placeholder
        //     }
        //     else {
        //         &self.placeholder
        //     }
        // }
        // else {
        //     data
        // };
        let content = data;

        self.selection = self.selection.constrain_to(data);

        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        let height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let selection_color = env.get(theme::SELECTION_COLOR);
        let cursor_color = env.get(theme::CURSOR_COLOR);

        let is_focused = ctx.focus_node().is_focused;

        // Paint the background
        let clip_rect = Size::new(self.width - BORDER_WIDTH, height).to_rect();

        // Render text, selection, and cursor inside a clip
        ctx.with_save(|rc| {
            rc.clip(clip_rect);

            // Calculate layout
            // let text_layout = self.get_layout(
            //     &mut rc.text(),
            //     &content,
            //     env,
            //     data.is_empty(),
            //     false,
            //     // self.selection.is_caret(),
            // );
            // let text_layout_selection = self.get_layout(&mut rc.text(), &content, env, false, true);
            let text_size = self.text.size();

            // Shift everything inside the clip by the hscroll_offset
            rc.transform(Affine::translate((-self.hscroll_offset, 0.)));

            // Layout, measure, and draw text
            // let text_height = font_size * 0.8;
            // let text_pos = Point::new(0.0 + PADDING_LEFT, text_height + PADDING_TOP);
            let top_padding = (height - text_size.height).min(PADDING_TOP).max(0.);
            let text_pos = Point::new(PADDING_LEFT, top_padding);

            // rc.draw_text(&text_layout, text_pos);
            self.text.draw(rc, text_pos);

            // Draw selection rect
            if !self.selection.is_caret() {
                for sel in self.text.rects_for_range(self.selection.range()) {
                    let sel = sel + text_pos.to_vec2();
                    let rounded = sel.to_rounded_rect(1.0);
                    rc.fill(rounded, &selection_color);
                }

                // let (left, right) = (self.selection.min(), self.selection.max());
                // let left_offset = self.x_for_offset(&text_layout, left);
                // let right_offset = self.x_for_offset(&text_layout, right);

                // let selection_width = right_offset - left_offset;

                // let selection_pos = Point::new(left_offset + PADDING_LEFT - 1., PADDING_TOP - 2.);

                // let selection_rect = RoundedRect::from_origin_size(
                //     selection_pos,
                //     Size::new(selection_width + 2., font_size + 4.).to_vec2(),
                //     1.,
                // );
                // rc.fill(selection_rect, &selection_color);
                // rc.clip(selection_rect);
                // rc.draw_text(&text_layout_selection, text_pos);
                // rc.clip(clip_rect);
            }

            // Paint the cursor if focused and there's no selection
            if is_focused && self.cursor_on {
                let line = self.text.cursor_line_for_text_position(self.cursor());
                let line = line + text_pos.to_vec2();
                rc.stroke(line, &cursor_color, 1.);
            }
        });

        // ctx.fill(clip_rect, &cursor_color);
        // Paint the border
        // ctx.stroke(clip_rect, &border_color, BORDER_WIDTH);
    }
}

impl Default for EditableText {
    fn default() -> Self {
        EditableText::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that when data is mutated externally widget
    /// can still be used to insert characters.
    #[test]
    fn data_can_be_changed_externally() {
        let mut widget = EditableText::new();
        let mut data = "".to_string();

        // First insert some chars
        widget.insert(&mut data, "o");
        widget.insert(&mut data, "n");
        widget.insert(&mut data, "e");

        assert_eq!("one", data);
        assert_eq!(3, widget.selection.start);
        assert_eq!(3, widget.selection.end);

        // Modify data externally (e.g data was changed in the parent widget)
        data = "".to_string();

        // Insert again
        widget.insert(&mut data, "a");
    }

    /// Test backspace on the combo character o̷
    #[test]
    fn backspace_combining() {
        let mut widget = EditableText::new();
        let mut data = "".to_string();

        widget.insert(&mut data, "\u{0073}\u{006F}\u{0337}\u{0073}");

        widget.delete_backward(&mut data);
        widget.delete_backward(&mut data);

        assert_eq!(data, String::from("\u{0073}\u{006F}"))
    }

    /// Devanagari codepoints are 3 utf-8 code units each.
    #[test]
    fn backspace_devanagari() {
        let mut widget = EditableText::new();
        let mut data = "".to_string();

        widget.insert(&mut data, "हिन्दी");
        widget.delete_backward(&mut data);
        assert_eq!(data, String::from("हिन्द"));
        widget.delete_backward(&mut data);
        assert_eq!(data, String::from("हिन्"));
        widget.delete_backward(&mut data);
        assert_eq!(data, String::from("हिन"));
        widget.delete_backward(&mut data);
        assert_eq!(data, String::from("हि"));
        widget.delete_backward(&mut data);
        assert_eq!(data, String::from("ह"));
        widget.delete_backward(&mut data);
        assert_eq!(data, String::from(""));
    }
}
