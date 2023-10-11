use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
            app.quit()
        },
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        _ => {},
    };
}