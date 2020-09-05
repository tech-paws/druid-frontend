use druid::{
    commands, AppDelegate, AppLauncher, Command, DelegateCtx, Env, Event, HotKey, KbKey,
    LocalizedString, Target, WindowDesc, WindowId,
};

use crate::theme;
use crate::ui::debug;
use crate::ui::scheme_editor;
use crate::ui::ui_state::UiState;

const WINDOW_TITLE: LocalizedString<UiState> = LocalizedString::new("Tech.Paws");

struct TechPawsAppDelegate {}

impl TechPawsAppDelegate {
    fn new() -> Self {
        TechPawsAppDelegate {}
    }
}

impl AppDelegate<UiState> for TechPawsAppDelegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        data: &mut UiState,
        _env: &Env,
    ) -> Option<Event> {
        match &event {
            Event::KeyDown(key_event) => {
                match key_event {
                    k_e if HotKey::new(None, KbKey::Character("`".into())).matches(k_e) => {
                        data.debug.show_terminal = !data.debug.show_terminal;
                        data.debug.terminal_command = "".into();
                        ctx.submit_command(
                            Command::new(commands::REQUEST_FOCUS, debug::TERMINAL_WIDGET_ID),
                            Target::Widget(debug::TERMINAL_WIDGET_ID),
                        );
                        None
                    }
                    _ => Some(event),
                }
            }
            _ => Some(event),
        }
    }
}

pub fn run() {
    let main_window = WindowDesc::new(scheme_editor::build_ui)
        .title(WINDOW_TITLE)
        .window_size((1024.0, 700.0));

    tech_paws_core::init_world();

    AppLauncher::with_window(main_window)
        .delegate(TechPawsAppDelegate::new())
        .configure_env(|env, _| theme::init(env))
        .launch(UiState::new())
        .expect("Failed to launch application");
}
